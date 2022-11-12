use std::collections::HashMap;

use log::{error, info};
use parry2d::{
    bounding_volume::{BoundingVolume, AABB},
    math::Point,
};

use crate::clients::client::Client;

use super::{sprite::Sprite, tile::Tile};

pub struct World {
    area: AABB,

    // TODO background?
    sprites: HashMap<usize, Sprite>,
    next_sprite_id: usize,

    events: Vec<Event>,
}

impl World {
    pub fn new() -> Self {
        Self {
            area: AABB::new_invalid(),
            sprites: HashMap::new(),
            next_sprite_id: 0,
            events: Vec::new(),
        }
    }

    pub fn get_sprite(&mut self, id: usize) -> &Sprite {
        // TODO return Option? Result?
        self.sprites.get(&id).unwrap()
    }

    pub fn create_sprite(&mut self, tile: &Tile) -> usize {
        let id = self.next_sprite_id;
        self.next_sprite_id = self.next_sprite_id + 1;

        self.sprites.insert(id, Sprite::new(tile));

        self.events.push(Event::SpriteCreated(id));

        id
    }

    pub fn move_sprite(&mut self, id: usize, x: f32, y: f32) {
        match self.sprites.get_mut(&id) {
            Some(sprite) => {
                sprite.set_position(x, y);
                self.events.push(Event::SpriteMoved(id));
            }
            None => error!("no sprite {id}"),
        }
    }

    pub fn fit_client_screens(&mut self, clients: &Vec<Client>) -> &AABB {
        self.area = AABB::new_invalid();

        for client in clients.iter() {
            self.area.merge(&client.screen().bounding_box());
        }

        &self.area
    }

    pub fn sync_clients(&mut self, clients: &mut Vec<Client>) {
        for event in self.events.iter() {
            info!("World event: {:?}", event);

            match event {
                Event::SpriteCreated(id) => {
                    for client in clients.iter_mut() {
                        let sprite = self.sprites.get(id).unwrap(); // TODO err

                        if client.screen().contains(&sprite.pos) {
                            let screen_pos = to_client_space(client, &sprite.pos);

                            client.draw_sprite(*id, sprite, screen_pos.x, screen_pos.y);
                        }
                    }
                }
                Event::SpriteDeleted(id) => {
                    //todo!()
                }
                Event::SpriteMoved(id) => {
                    for client in clients.iter_mut() {
                        let sprite = self.sprites.get(id).unwrap(); // TODO err

                        if client.screen().contains(&sprite.pos) {
                            let screen_pos = to_client_space(client, &sprite.pos);

                            client.draw_sprite(*id, sprite, screen_pos.x, screen_pos.y);
                        }
                    }
                }
            }
        }

        self.events.clear();
    }
}

#[derive(Debug)]
enum Event {
    SpriteCreated(usize),
    SpriteDeleted(usize),
    SpriteMoved(usize),
}

fn to_client_space(client: &Client, world_pos: &Point<f32>) -> Point<u8> {
    Point::new(
        ((world_pos.x - client.screen().pos.x) / client.screen().size.x
            * (client.screen().res.x as f32)) as u8,
        ((world_pos.y - client.screen().pos.y) / client.screen().size.y
            * (client.screen().res.y as f32)) as u8,
    )
}
