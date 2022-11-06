use parry2d::bounding_volume::AABB;
use parry2d::math::{Point, Vector};
use std::ops::Add;

pub struct Screen {
    pub pos: Point<f32>,
    pub size: Vector<f32>, // TODO store as diagonal to avoid ratio inaccuracies?
    pub res: Vector<usize>,
}

impl Screen {
    pub fn bounding_box(&self) -> AABB {
        AABB::new(self.pos, self.pos.add(self.size))
    }

    pub fn contains(&self, point: &Point<f32>) -> bool {
        self.bounding_box().contains_local_point(point)
    }
}
