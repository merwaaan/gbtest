use crate::commands::Command;
use std::{io::Write, net::TcpStream};

pub struct Client {
    stream: TcpStream,

    command_buffer: Vec<Command>,

    // Bits: Start Select B A Down Up Left Right
    inputs: u8,
}

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

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            command_buffer: Vec::new(),
            inputs: 0,
        }
    }

    pub fn id(&self) -> String {
        // TODO avoid String alloc
        self.stream.peer_addr().unwrap().to_string()
    }

    pub fn buffer_command(&mut self, command: Command) {
        self.command_buffer.push(command);
    }

    pub fn send_commands(&mut self) {
        if self.command_buffer.is_empty() {
            return;
        }

        println!("Sending {} commands", self.command_buffer.len());

        self.stream.write(&[0x45]).unwrap();

        self.stream
            .write(&[self.command_buffer.len() as u8])
            .unwrap();

        for command in self.command_buffer.iter() {
            println!("Sending command: {:?}", command);

            let data = command.to_bytes();
            self.stream.write(&data).unwrap();
        }

        self.command_buffer.clear();
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
