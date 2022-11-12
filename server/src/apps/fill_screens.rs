use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::time::Duration;

use crate::apps::App;
use crate::clients::client::Client;
use crate::engine::color::{BLACK, WHITE};
use crate::engine::tile::Tile;

struct ClientInfo {
    filled_tiles: usize,
    filling: bool, // Or erasing
    time_since_last_tile: Duration,
}

pub struct FillScreensApp {
    clients_info: HashMap<u8, ClientInfo>,
    new_tile_delay: Duration, // Tile/s
}

impl FillScreensApp {
    pub fn new() -> Self {
        let delay = Duration::from_millis(100);

        Self {
            clients_info: HashMap::new(),
            new_tile_delay: delay,
        }
    }
}

lazy_static! {
    static ref FILLED_TILE: Tile = Tile::filled(8, 8, BLACK);
    static ref EMPTY_TILE: Tile = Tile::filled(8, 8, WHITE);
}

impl App for FillScreensApp {
    fn update(&mut self, dt: &Duration, clients: &mut Vec<Client>) {
        for client in clients.iter_mut() {
            // Initialize new clients

            if !self.clients_info.contains_key(&client.id()) {
                self.clients_info.insert(
                    client.id(),
                    ClientInfo {
                        filled_tiles: 0,
                        filling: true,
                        time_since_last_tile: Duration::ZERO,
                    },
                );

                // Clear the screen
                for tile_y in 0..(client.screen().res.y / 8) as u8 {
                    for tile_x in 0..(client.screen().res.x / 8) as u8 {
                        // TODO generalize res
                        client.draw_tile(&EMPTY_TILE, tile_x * 8, tile_y * 8);
                    }
                }
            }

            let client_info = self.clients_info.get_mut(&client.id()).unwrap();

            // Progressively fill the screen with tiles

            client_info.time_since_last_tile = client_info.time_since_last_tile.add(*dt);

            while client_info.time_since_last_tile > self.new_tile_delay {
                // Screen filled: restart with the other tile to progressively clear/fill

                let max_tile_x = client.screen().res.x / 8;
                let max_tile_y = client.screen().res.y / 8;
                let max_tile_count = max_tile_x * max_tile_y;

                if client_info.filled_tiles >= max_tile_count {
                    client_info.filled_tiles = 0;
                    client_info.filling = !client_info.filling;
                }

                // Add one tile

                let tile_x = (client_info.filled_tiles % max_tile_x) as u8;
                let tile_y = (client_info.filled_tiles / max_tile_x) as u8;

                let tile: &Tile = if client_info.filling {
                    &FILLED_TILE
                } else {
                    &EMPTY_TILE
                };

                client.draw_tile(tile, tile_x * 8, tile_y * 8);

                client_info.filled_tiles += 1;

                client_info.time_since_last_tile =
                    client_info.time_since_last_tile.sub(self.new_tile_delay);
            }
        }
    }
}
