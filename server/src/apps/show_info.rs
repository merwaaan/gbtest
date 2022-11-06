use std::collections::HashMap;
use std::time::Duration;

use parry2d::math::Point;

use crate::apps::App;
use crate::clients::client::Client;
use crate::commands::ClientCommand;

struct Info {
    pos: Point<f32>,
}

pub struct ShowInfoApp {
    last_client_info: HashMap<u8, Info>,
}

impl ShowInfoApp {
    pub fn new() -> Self {
        Self {
            last_client_info: HashMap::new(),
        }
    }
}

impl App for ShowInfoApp {
    fn update(&mut self, _dt: &Duration, clients: &mut Vec<Client>) {
        for client in clients {
            let mut needs_update = false;

            match self.last_client_info.get_mut(&client.id()) {
                // The client is new
                None => {
                    println!("ShowInfoApp: new client {}", client.id());

                    self.last_client_info.insert(
                        client.id(),
                        Info {
                            pos: client.screen().pos,
                        },
                    );
                    needs_update = true;
                }
                // The client has already been processed but its attributes changed
                Some(info) => {
                    if info.pos != client.screen().pos {
                        println!("ShowInfoApp: client {} attributes changed", client.id());

                        info.pos = client.screen().pos;
                        needs_update = true;
                    }
                }
            }

            if needs_update {
                client.buffer_command(ClientCommand::ClearScreen);
                client.buffer_command(ClientCommand::PrintText(
                    0,
                    0,
                    format!(
                        "ID: {} / Pos: {} {}",
                        client.id(),
                        client.screen().pos.x,
                        client.screen().pos.y
                    ),
                ));
            }
        }
    }
}
