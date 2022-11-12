use parry2d::na::Vector2;

use super::color::Color;

#[derive(Clone, Hash)]
pub struct Tile {
    pub size: Vector2<u8>,
    pub pixels: Vec<Color>,
}

impl Tile {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            size: Vector2::new(width, height),
            pixels: Vec::with_capacity(width as usize * height as usize),
        }
    }

    pub fn filled(width: u8, height: u8, color: Color) -> Self {
        Self {
            size: Vector2::new(width, height),
            pixels: vec![color; width as usize * height as usize],
        }
    }

    pub fn from_pixels(width: u8, height: u8, pixels: Vec<Color>) -> Self {
        assert!(pixels.len() == width as usize * height as usize);

        Self {
            size: Vector2::new(width, height),
            pixels,
        }
    }
}
