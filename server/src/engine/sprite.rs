use parry2d::math::Point;

use super::tile::Tile;

pub struct Sprite {
    pub tile: Tile,
    pub pos: Point<f32>,
}

impl Sprite {
    pub fn new(tile: &Tile) -> Self {
        Self {
            tile: tile.clone(),
            pos: Point::new(0.0, 0.0),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }
}
