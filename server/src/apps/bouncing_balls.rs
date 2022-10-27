use std::ops::Add;

use super::app::App;
use crate::client::Client;
use crate::ServerCommand;
use crate::commands::ClientCommand;
use parry2d::{bounding_volume::{AABB, BoundingVolume}, math::{Point, Vector}};

struct Ball {
    pos: Point<f32>,
    vel: Vector<f32>
    // TODO radius?
}

pub struct BouncingBallsApp {
    area: AABB,
    balls: Vec<Ball>
}

impl BouncingBallsApp {
    pub fn new() -> Self {
        Self {
            area: AABB::new_invalid(),
            balls: Vec::new()
        }
    }
}

impl App for BouncingBallsApp {
    fn update(&mut self, clients: &mut Vec<Client>) {
        // Compute the current bounding box

        let mut new_aabb = AABB::new_invalid();

        for client in clients.iter() {
            let (x, y, w, h) = client.screen();

            let client_aabb = AABB::new(
                Point::new(*x, *y),
                Point::new(*x + *w, *y + *h)
            );

            new_aabb.merge(&client_aabb);
        }

        // Check if it changed, correct the balls' positions if needed
        
        if new_aabb != self.area {
            self.area = new_aabb;

            // TODO correct
        }

        // TEMP Spawn a ball
        // TODO on input?
        
        if self.balls.is_empty() && self.area.volume() > 0.0 {
            self.balls.push(
                Ball {
                    pos: self.area.center(),
                    vel: Vector::new(1.0, 1.0)
                }
            )
        }

        // Move the balls

        for ball in &mut self.balls {
            // Roll

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

        // Send to clients

        for client in clients.iter() {
            for ball in &self.balls {
                // TODO check if overlaps screen
                // TODO convert to screen-space
                //client.buffer_command(ClientCommand::DrawPoint(?, ?));
            }
        }
    }
    
    fn process_server_command(&mut self, command: &ServerCommand) {               
    }
}
