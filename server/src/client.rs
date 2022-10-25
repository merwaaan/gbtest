use crate::commands::Command;
use std::io::{self, Read};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
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

pub struct Client {
    id: String,

    thread: JoinHandle<()>,
    //stream: TcpStream,
    unstaged_commands: Vec<Command>,
    staged_commands: Arc<Mutex<Vec<Command>>>,

    // Bits: Start Select B A Down Up Left Right
    inputs: u8,
}

impl Client {
    pub fn new(mut stream: TcpStream) -> Self {
        let id = stream.peer_addr().unwrap().to_string();

        let concurrent_commands = Arc::new(Mutex::new(Vec::new()));
        let staged_command_buffer = concurrent_commands.clone();

        let thread = thread::spawn(move || {
            {
                // Send commands

                let commands: MutexGuard<Vec<Command>> = concurrent_commands.lock().unwrap();

                println!("Sending {} commands", commands.len());

                println!("0");

                stream.write(&[commands.len() as u8]).unwrap();

                for command in commands.iter() {
                    println!("Sending command: {:?}", command);

                    let data = command.to_bytes();
                    stream.write(&data).unwrap();
                }

                concurrent_commands.lock().unwrap().clear();
            }

            // Receive inputs

            let mut received_data = [0u8];

            println!("1");
            let received_bytes = match stream.read(&mut received_data) {
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
            println!("2");
        });

        Self {
            id,
            thread,
            //stream,
            unstaged_commands: Vec::new(),
            staged_commands: staged_command_buffer,
            inputs: 0,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn buffer_command(&mut self, command: Command) {
        self.unstaged_commands.push(command);
    }

    pub fn send_commands(&mut self) {
        let mut staged_commands: MutexGuard<Vec<Command>> = self.staged_commands.lock().unwrap();

        for unstaged_command in self.unstaged_commands.iter_mut() {
            println!("Staging command: {:?}", unstaged_command);

            staged_commands.push(unstaged_command.clone());
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
