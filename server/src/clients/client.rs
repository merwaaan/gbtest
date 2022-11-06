use serde::{Deserialize, Serialize};

use crate::clients::gameboy::GameBoyDriver;
use crate::clients::gameboycolor::GameBoyColorDriver;
use crate::clients::screen::Screen;
use crate::commands::ClientCommand;
use crate::ServerCommand;
use core::panic;
use std::fs;
use std::io::{self, Read};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{io::Write, net::TcpStream};

use super::driver::Driver;

pub enum Button {
    Start,
    Select,
    B,
    A,
    Down,
    Up,
    Left,
    Right,
}

pub struct Client {
    id: u8,

    driver: Box<dyn Driver + Send>,

    thread: JoinHandle<()>,

    unstaged_commands: Vec<ClientCommand>,
    staged_commands: Arc<Mutex<Vec<ClientCommand>>>,

    // Bits: Start Select B A Down Up Left Right
    inputs: Arc<Mutex<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientAttributes {
    pub pos: (f32, f32),
}

impl ClientAttributes {
    fn new(client: &Client) -> Self {
        Self {
            pos: (client.screen().pos.x, client.screen().pos.y),
        }
    }
}

static NEXT_ID: AtomicU8 = AtomicU8::new(0);

fn client_filename(id: u8) -> String {
    format!("client-{}.json", id)
}

impl Client {
    pub fn from_stream(mut stream: TcpStream) -> Self {
        // First, receive the system ID from the client

        let mut received_data = [0u8];

        let system_id = match stream.read(&mut received_data) {
            Ok(_) => received_data[0],
            Err(e) => {
                if e.kind() != io::ErrorKind::WouldBlock {
                    println!("Client error: {}", e);
                }

                0
            }
        };

        let mut driver: Box<dyn Driver + Send> = match system_id {
            0 => Box::new(GameBoyDriver::new()),
            1 => Box::new(GameBoyColorDriver::new()),
            _ => panic!("unknown system ID {system_id}"),
        };

        println!("{system_id}");

        // Attribute a client ID

        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

        // Try to read saved attributes
        match fs::read_to_string(client_filename(id)) {
            Ok(json_string) => match serde_json::from_str::<ClientAttributes>(&json_string) {
                Ok(attributes) => {
                    driver.screen_mut().pos.x = attributes.pos.0;
                    driver.screen_mut().pos.y = attributes.pos.1;
                }
                Err(e) => {
                    println!("Cannot parse attributes: {}", e);
                }
            },
            Err(e) => {
                println!("Cannot load client attributes: {}", e);
            }
        }

        let concurrent_staged_commands = Arc::new(Mutex::new(Vec::new()));
        let staged_commands = concurrent_staged_commands.clone();

        let concurrent_inputs = Arc::new(Mutex::new(0u8));
        let inputs = concurrent_inputs.clone();

        let thread = thread::spawn(move || {
            loop {
                {
                    // Send commands

                    let mut commands: MutexGuard<Vec<ClientCommand>> =
                        concurrent_staged_commands.lock().unwrap();

                    stream.write(&[commands.len() as u8]).unwrap();

                    for command in commands.iter() {
                        println!("Sending command: {:?} = {:?}", command, command.to_bytes());

                        let data = command.to_bytes();
                        stream.write(&data).unwrap();
                    }

                    commands.clear();
                }

                // Receive inputs

                let mut received_data = [0u8];

                *concurrent_inputs.lock().unwrap() = match stream.read(&mut received_data) {
                    Ok(_) => received_data[0],
                    Err(e) => {
                        if e.kind() != io::ErrorKind::WouldBlock {
                            println!("Client error: {}", e);
                        }

                        0
                    }
                };

                thread::sleep(Duration::from_millis(20));
            }
        });

        Self {
            id,
            driver,
            thread,
            unstaged_commands: Vec::new(),
            staged_commands,
            inputs,
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn screen(&self) -> &Screen {
        &self.driver.screen()
    }

    pub fn buffer_command(&mut self, command: ClientCommand) {
        self.unstaged_commands.push(command);
    }

    pub fn send_commands(&mut self) {
        let mut concurrent_staged_commands: MutexGuard<Vec<ClientCommand>> =
            self.staged_commands.lock().unwrap();

        for unstaged_command in self.unstaged_commands.iter() {
            //println!("Staging command: {:?}", unstaged_command);

            concurrent_staged_commands.push(unstaged_command.clone());
        }

        self.unstaged_commands.clear();
    }

    pub fn process_server_command(&mut self, command: &ServerCommand) {
        match command {
            ServerCommand::Pos { client_id, x, y } => {
                if self.id == *client_id {
                    println!("client {}: pos to {} {}", self.id, x, y);
                    self.driver.screen_mut().pos.x = *x;
                    self.driver.screen_mut().pos.y = *y;

                    // Save
                    match serde_json::to_string(&ClientAttributes::new(self)) {
                        Ok(json_string) => match fs::write(client_filename(self.id), json_string) {
                            Err(e) => {
                                println!("Cannot save client attributes: {}", e);
                            }
                            _ => {}
                        },
                        Err(e) => {
                            println!("Cannot serialize attributes: {}", e);
                        }
                    }
                }
            }

            _ => {}
        }
    }

    pub fn button_pressed(&self, button: Button) -> bool {
        let concurrent_inputs = self.inputs.lock().unwrap();

        match button {
            Button::Start => (*concurrent_inputs & 0x80) != 0,
            Button::Select => (*concurrent_inputs & 0x40) != 0,
            Button::B => (*concurrent_inputs & 0x20) != 0,
            Button::A => (*concurrent_inputs & 0x10) != 0,
            Button::Down => (*concurrent_inputs & 0x08) != 0,
            Button::Up => (*concurrent_inputs & 0x04) != 0,
            Button::Left => (*concurrent_inputs & 0x02) != 0,
            Button::Right => (*concurrent_inputs & 0x01) != 0,
        }
    }
}
