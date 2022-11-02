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
}

impl BouncingBallsApp {
    pub fn new() -> Self {
        Self {
            area: AABB::new_invalid(),
            balls: Vec::new(),
        }
    }
}

impl App for BouncingBallsApp {
    fn update(&mut self, dt: &Duration, clients: &mut Vec<Client>) {
        // Compute the current bounding box

        let mut new_aabb = AABB::new_invalid();

        for client in clients.iter() {
            new_aabb.merge(&client.screen().bounding_box());
        }

        // Check if it changed, correct the balls' positions if needed

        if new_aabb != self.area {
            self.area = new_aabb;

            // TODO correct
        }

        // TEMP Spawn a ball
        // TODO on input?

        if self.balls.is_empty() && self.area.volume() > 0.0 {
            self.balls.push(Ball {
                pos: self.area.center(),
                vel: Vector::new(1.0, 1.0),
            })
        }

        // Clear the previous balls

        for client in clients.iter_mut() {
            for ball in &self.balls {
                // TODO check if overlaps screen

                let screen_pos = to_client_space(client, &ball.pos);
                client.buffer_command(ClientCommand::ClearRect(screen_pos.x, screen_pos.y, 1, 1));
            }
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

        // Draw at the new position

        for client in clients.iter_mut() {
            for ball in &self.balls {
                // TODO check if overlaps screen

                let screen_pos = to_client_space(client, &ball.pos);
                client.buffer_command(ClientCommand::DrawPoint(screen_pos.x, screen_pos.y));
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
