use image::DynamicImage;

use crate::engine::{sprite::Sprite, tile::Tile};

use super::{client::CommandData, screen::Screen};

pub trait Driver {
    fn screen(&self) -> &Screen;
    fn screen_mut(&mut self) -> &mut Screen;

    //

    fn draw_text(&mut self, _text: &str, _x: u32, _y: u32) -> Vec<CommandData> {
        unimplemented!()
    }

    fn draw_tile(&mut self, _tile: &Tile, _x: u8, _y: u8) -> Vec<CommandData> {
        unimplemented!()
    }

    fn draw_image(&mut self, _image: &DynamicImage) -> Vec<CommandData> {
        unimplemented!()
    }

    fn draw_sprite(&mut self, id: usize, sprite: &Sprite, x: u8, y: u8) -> Vec<CommandData> {
        unimplemented!()
    }
}
