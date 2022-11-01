use parry2d::bounding_volume::AABB;
use parry2d::math::{Point, Vector};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::commands::ClientCommand;
use crate::ServerCommand;
use std::fs;
use std::io::{self, Read};
use std::ops::Add;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{io::Write, net::TcpStream};

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

pub struct Screen {
    pub pos: Point<f32>,
    pub size: Vector<f32>,
    pub res: Vector<usize>,
}

impl Screen {
    pub fn gameboy() -> Self {
        Self {
            pos: Point::new(0.0, 0.0),
            size: Vector::new(14.8, 9.0),
            res: Vector::new(160, 144),
        }
    }

    pub fn bounding_box(&self) -> AABB {
        AABB::new(self.pos, self.pos.add(self.size))
    }
}

pub struct Client {
    id: u8,

    screen: Screen,

    thread: JoinHandle<()>,

    unstaged_commands: Vec<ClientCommand>,
    staged_commands: Arc<Mutex<Vec<ClientCommand>>>,

    // Bits: Start Select B A Down Up Left Right
    inputs: u8,
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
    pub fn new(mut stream: TcpStream) -> Self {
        // Attribute an ID
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

        let mut screen = Screen::gameboy();

        // Try to read saved attributes
        match fs::read_to_string(client_filename(id)) {
            Ok(json_string) => match serde_json::from_str::<ClientAttributes>(&json_string) {
                Ok(attributes) => {
                    screen.pos.x = attributes.pos.0;
                    screen.pos.y = attributes.pos.1;
                }
                Err(e) => {
                    println!("Cannot parse attributes");
                }
            },
            Err(e) => {
                println!("Cannot load client attributes");
            }
        }

        let concurrent_staged_commands = Arc::new(Mutex::new(Vec::new()));
        let staged_commands = concurrent_staged_commands.clone();

        let thread = thread::spawn(move || {
            loop {
                {
                    // Send commands

                    let mut commands: MutexGuard<Vec<ClientCommand>> =
                        concurrent_staged_commands.lock().unwrap();

                    stream.write(&[commands.len() as u8]).unwrap();

                    for command in commands.iter() {
                        println!("Sending command: {:?}", command);

                        let data = command.to_bytes();
                        stream.write(&data).unwrap();
                    }

                    commands.clear();
                }

                // Receive inputs

                let mut received_data = [0u8];

                match stream.read(&mut received_data) {
                    Ok(n) => {
                        println!("Received {} bytes", n);
                        n
                    }
                    Err(e) => {
                        if e.kind() != io::ErrorKind::WouldBlock {
                            println!("Error: {}", e);
                        }

                        0
                    }
                };

                thread::sleep(Duration::from_millis(1000));
            }
        });

        Self {
            id,
            screen,
            thread,
            unstaged_commands: Vec::new(),
            staged_commands,
            inputs: 0,
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn buffer_command(&mut self, command: ClientCommand) {
        self.unstaged_commands.push(command);
    }

    pub fn send_commands(&mut self) {
        let mut concurrent_staged_commands: MutexGuard<Vec<ClientCommand>> =
            self.staged_commands.lock().unwrap();

        for unstaged_command in self.unstaged_commands.iter() {
            println!("Staging command: {:?}", unstaged_command);

            concurrent_staged_commands.push(unstaged_command.clone());
        }

        self.unstaged_commands.clear();
    }

    pub fn process_server_command(&mut self, command: &ServerCommand) {
        match command {
            ServerCommand::Pos { client_id, x, y } => {
                if self.id == *client_id {
                    println!("client {}: pos to {} {}", self.id, x, y);
                    self.screen.pos.x = *x;
                    self.screen.pos.y = *y;

                    // Save
                    match serde_json::to_string(&ClientAttributes::new(self)) {
                        Ok(json_string) => match fs::write(client_filename(self.id), json_string) {
                            Err(e) => {
                                println!("Cannot save client attributes");
                            }
                            _ => {}
                        },
                        Err(e) => {
                            println!("Cannot serialize attributes");
                        }
                    }
                }
            }

            _ => {}
        }
    }

    pub fn button_pressed(&self, button: Button) -> bool {
        match button {
            Button::Start => (self.inputs & 0x80) != 0,
            Button::Select => (self.inputs & 0x40) != 0,
            Button::B => (self.inputs & 0x20) != 0,
            Button::A => (self.inputs & 0x10) != 0,
            Button::Down => (self.inputs & 0x08) != 0,
            Button::Up => (self.inputs & 0x04) != 0,
            Button::Left => (self.inputs & 0x02) != 0,
            Button::Right => (self.inputs & 0x01) != 0,
        }
    }
}
