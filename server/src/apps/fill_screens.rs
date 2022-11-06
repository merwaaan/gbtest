use std::collections::HashMap;
use std::time::Duration;

use crate::apps::App;
use crate::clients::client::{Button, Client};
use crate::commands::ClientCommand;

struct ClientInfo {
    pixel: usize,
    speed: u8,
}

pub struct FillScreensApp {
    clients_info: HashMap<u8, ClientInfo>,
}

impl FillScreensApp {
    pub fn new() -> Self {
        Self {
            clients_info: HashMap::new(),
        }
    }
}

impl App for FillScreensApp {
    fn update(&mut self, dt: &Duration, clients: &mut Vec<Client>) {
        for client in clients.iter_mut() {
            // Initialize new clients

            if !self.clients_info.contains_key(&client.id()) {
                self.clients_info.insert(
                    client.id(),
                    ClientInfo {
                        pixel: 0,
                        speed: 20,
                    },
                );

                client.buffer_command(ClientCommand::ClearScreen);
            }

            let client_info = self.clients_info.get_mut(&client.id()).unwrap();

            // Press Start: clear

            if client.button_pressed(Button::Start) {
                client_info.pixel = 0;
                client.buffer_command(ClientCommand::ClearScreen);
            }

            // Fill one pixels

            for _ in 0..client_info.speed {
                if client_info.pixel < 160 * 144 {
                    // TODO generalize res
                    client_info.pixel += 1;

                    let x = (client_info.pixel % 160) as u8;
                    let y = (client_info.pixel / 160) as u8;

                    client.buffer_command(ClientCommand::DrawPoint(x, y));
                }
            }
        }
    }
}
