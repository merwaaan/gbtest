use super::{driver::Driver, screen::Screen};

use parry2d::math::{Point, Vector};

pub struct GameBoyDriver {
    screen: Screen,
}

impl GameBoyDriver {
    pub fn new() -> Self {
        Self {
            screen: Screen {
                pos: Point::new(0.0, 0.0),
                size: Vector::new(4.8, 4.3), // TODO store as diagonal to avoid ratio inaccuracies?
                res: Vector::new(160, 144),
            },
        }
    }
}

impl Driver for GameBoyDriver {
    fn screen(&self) -> &Screen {
        &self.screen
    }

    fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }
}
