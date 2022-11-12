use std::collections::HashSet;
use std::f32::INFINITY;
use std::{ops::Add, time::Duration};

use crate::apps::App;
use crate::clients::client::Client;
use crate::engine::color::{BLUE, RED, WHITE};
use crate::engine::tile::Tile;
use crate::engine::world::World;
use parry2d::math::{Point, Vector};

struct Ball {
    sprite_id: usize,
    vel: Vector<f32>,
    // TODO radius?
}

pub struct BouncingBallsApp {
    world: World,
    balls: Vec<Ball>,
    known_client_ids: HashSet<u8>,
}

impl BouncingBallsApp {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            balls: Vec::new(),
            known_client_ids: HashSet::new(),
        }
    }
}

lazy_static! {
    static ref BALL_TILE: Tile = Tile::from_pixels(
        8,
        8,
        vec![
            WHITE, WHITE, WHITE, RED,  RED,  WHITE, WHITE, WHITE, //
            WHITE, WHITE, RED,   BLUE, BLUE, RED,   WHITE, WHITE, //
            WHITE, RED,   BLUE,  BLUE, BLUE, BLUE,  RED,   WHITE, //
            RED,   BLUE,  BLUE,  BLUE, BLUE, BLUE,  BLUE,  RED, //
            RED,   BLUE,  BLUE,  BLUE, BLUE, BLUE,  BLUE,  RED, //
            WHITE, RED,   BLUE,  BLUE, BLUE, BLUE,  RED,   WHITE, //
            WHITE, WHITE, RED,   BLUE, BLUE, RED,   WHITE, WHITE, //
            WHITE, WHITE, WHITE, RED,  RED,  WHITE, WHITE, WHITE
            ]
    );
}

impl App for BouncingBallsApp {
    fn update(&mut self, dt: &Duration, clients: &mut Vec<Client>) {
        let area = self.world.fit_client_screens(clients).clone();
        // TODO correct ball pos when area changes

        // Spawn the first ball
        // TODO more on input?

        if self.balls.is_empty() && area.volume() != INFINITY {
            self.balls.push(Ball {
                sprite_id: self.world.create_sprite(&BALL_TILE),
                vel: Vector::new(0.1, 0.1),
            });
        }

        // Move the balls
        // TODO dt

        for ball in &mut self.balls {
            let mut pos = self.world.get_sprite(ball.sprite_id).pos;

            pos = pos.add(ball.vel);

            // Bounce
            // TODO play sound on client containing ball

            if pos.x < area.mins.x {
                pos.x = area.mins.x;
                ball.vel.x *= -1.0;
            }
            if pos.x > area.maxs.x {
                pos.x = area.maxs.x;
                ball.vel.x *= -1.0;
            }
            if pos.y < area.mins.y {
                pos.y = area.mins.y;
                ball.vel.y *= -1.0;
            }
            if pos.y > area.maxs.y {
                pos.y = area.maxs.y;
                ball.vel.y *= -1.0;
            }

            self.world.move_sprite(ball.sprite_id, pos.x, pos.y);
        }

        self.world.sync_clients(clients);
    }
}
