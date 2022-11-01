use std::collections::HashMap;
use std::time::Duration;

use crate::apps::App;
use crate::client::{Button, Client};
use crate::commands::ClientCommand;

pub struct FillScreensApp {
    pixels: HashMap<u8, usize>,
}

impl FillScreensApp {
    pub fn new() -> Self {
        Self {
            pixels: HashMap::new(),
        }
    }
}

impl App for FillScreensApp {
    fn update(&mut self, dt: &Duration, clients: &mut Vec<Client>) {
        for client in clients.iter_mut() {
            // Initialize new clients

            if !self.pixels.contains_key(&client.id()) {
                self.pixels.insert(client.id(), 0);

                client.buffer_command(ClientCommand::ClearScreen);
            }

            // Press Start: clear

            if client.button_pressed(Button::Start) {
                *self.pixels.get_mut(&client.id()).unwrap() = 0;

                client.buffer_command(ClientCommand::ClearScreen);
            }

            // Fill one pixels

            let pixel = self.pixels.get_mut(&client.id()).unwrap();

            if *pixel < 160 * 144 {
                // TODO generalize res
                *pixel += 1;

                let x = (*pixel % 144) as u8;
                let y = (*pixel / 160) as u8;

                client.buffer_command(ClientCommand::DrawPoint(x, y));
            }
        }
    }
}
