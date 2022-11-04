use std::collections::HashSet;
use std::{ops::Add, time::Duration};

use crate::client::Client;
use crate::{apps::App, commands::ClientCommand};
use parry2d::{
    bounding_volume::{BoundingVolume, AABB},
    math::{Point, Vector},
};

struct Ball {
    pos: Point<f32>,
    vel: Vector<f32>,
    // TODO radius?
}

pub struct BouncingBallsApp {
    area: AABB,
    balls: Vec<Ball>,
    known_client_ids: HashSet<u8>,
}

impl BouncingBallsApp {
    pub fn new() -> Self {
        Self {
            area: AABB::new_invalid(),
            balls: Vec::new(),
            known_client_ids: HashSet::new(),
        }
    }
}

static ball_tile: [u8; 16] = [
    0x3Cu8, 0x3Cu8, 0x3Cu8, 0x66u8, 0xFFu8, 0xFFu8, 0xFFu8, 0xBDu8, //
    0xFFu8, 0xBDu8, 0xFFu8, 0xFFu8, 0x3Cu8, 0x66u8, 0x3Cu8, 0x3Cu8,
];

static wall_tile: [u8; 16] = [
    0xFFu8, 0xFFu8, 0x81u8, 0x81u8, 0x81u8, 0x81u8, 0x81u8, 0x81u8, //
    0x81u8, 0x81u8, 0x81u8, 0x81u8, 0x81u8, 0x81u8, 0xFFu8, 0xFFu8,
];

impl App for BouncingBallsApp {
    fn update(&mut self, dt: &Duration, clients: &mut Vec<Client>) {
        // Compute the current bounding box

        let mut new_aabb = AABB::new_invalid();

        for client in clients.iter_mut() {
            new_aabb.merge(&client.screen().bounding_box());
        }

        if new_aabb != self.area {
            self.area = new_aabb;

            // TODO correct ball pos
        }

        // TEMP Spawn a ball
        // TODO on input?

        if self.balls.is_empty() && self.area.volume() > 0.0 {
            self.balls.push(Ball {
                pos: self.area.center(),
                vel: Vector::new(0.1, 0.1),
            });

            // TODO create sprites for clients
        }

        // Handle new clients

        for client in clients.iter_mut() {
            if !self.known_client_ids.contains(&client.id()) {
                self.known_client_ids.insert(client.id());

                // Load the tiles

                client.buffer_command(ClientCommand::LoadTile(false, 0, ball_tile));
                client.buffer_command(ClientCommand::LoadTile(true, 1, wall_tile));

                // Create the ball sprites

                for ball_index in 0..self.balls.len() {
                    client.buffer_command(ClientCommand::SetSpriteTile(ball_index as u8, 0));
                }

                // Create the walls in the background

                client.buffer_command(ClientCommand::SetBackgroundTile(0, 0, 1));
            }
        }

        // Move the balls

        for ball in &mut self.balls {
            ball.pos = ball.pos.add(ball.vel);

            // Bounce

            if ball.pos.x < self.area.mins.x {
                ball.pos.x = self.area.mins.x;
                ball.vel.x *= -1.0;
            }
            if ball.pos.x > self.area.maxs.x {
                ball.pos.x = self.area.maxs.x;
                ball.vel.x *= -1.0;
            }
            if ball.pos.y < self.area.mins.y {
                ball.pos.y = self.area.mins.y;
                ball.vel.y *= -1.0;
            }
            if ball.pos.y > self.area.maxs.y {
                ball.pos.y = self.area.maxs.y;
                ball.vel.y *= -1.0;
            }
        }

        // Update the sprites positions

        for client in clients.iter_mut() {
            for (ball_index, ball) in self.balls.iter().enumerate() {
                if client.screen().contains(&ball.pos) {
                    let screen_pos = to_client_space(client, &ball.pos);

                    client.buffer_command(ClientCommand::MoveSprite(
                        ball_index as u8,
                        screen_pos.x,
                        screen_pos.y,
                    ));
                }
            }
        }
    }
}

fn to_client_space(client: &Client, world_pos: &Point<f32>) -> Point<u8> {
    Point::new(
        ((world_pos.x - client.screen().pos.x) / client.screen().size.x
            * (client.screen().res.x as f32)) as u8,
        ((world_pos.y - client.screen().pos.y) / client.screen().size.y
            * (client.screen().res.y as f32)) as u8,
    )
}
