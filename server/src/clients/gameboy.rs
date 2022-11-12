use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
};

use crate::engine::{sprite::Sprite, tile::Tile};

use super::{client::CommandData, driver::Driver, screen::Screen};

use image::DynamicImage;
use log::info;
use parry2d::math::{Point, Vector};

pub struct GameBoyDriver {
    screen: Screen,

    loaded_tile_indices: HashMap<u64, u8>,
    next_tile_index: u8,

    loaded_sprite_indices: HashSet<usize>,
}

impl GameBoyDriver {
    pub fn new() -> Self {
        Self {
            screen: Screen {
                pos: Point::new(0.0, 0.0),
                size: Vector::new(4.8, 4.3), // TODO store as diagonal to avoid ratio inaccuracies?
                res: Vector::new(160, 144),
            },
            loaded_tile_indices: HashMap::new(),
            next_tile_index: 1, // TEMP =1 to avoid filling bg, switch back to 0 later
            loaded_sprite_indices: HashSet::new(),
        }
    }

    fn load_tile_if_needed(&mut self, tile: &Tile) -> (Vec<CommandData>, u8) {
        // TODO warn if size != 8x8
        // TODO warn if tile_index > max

        let mut commands = Vec::new();

        // Re-use the tile if it has already been loaded

        let mut hasher = DefaultHasher::new();
        tile.hash(&mut hasher);
        let tile_hash = hasher.finish();

        let tile_index = match self.loaded_tile_indices.get(&tile_hash) {
            Some(tile_index) => *tile_index,
            None => {
                let this_tile_index = self.next_tile_index;

                info!("loading tile");

                commands.push(command_load_tiles(
                    true,
                    this_tile_index as u16, /* TODO u8 good enough? */
                    1,
                    tile_to_gb(tile),
                ));

                self.next_tile_index = self.next_tile_index + 1;
                self.loaded_tile_indices.insert(tile_hash, this_tile_index);

                this_tile_index
            }
        };

        (commands, tile_index)
    }
}

impl Driver for GameBoyDriver {
    fn screen(&self) -> &Screen {
        &self.screen
    }

    fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    // High-level commands

    fn draw_text(&mut self, text: &str, x: u32, y: u32) -> Vec<CommandData> {
        vec![command_draw_text(text, x, y)]
    }

    fn draw_tile(&mut self, tile: &Tile, x: u8, y: u8) -> Vec<CommandData> {
        // Load the tile

        let (mut commands, tile_index) = self.load_tile_if_needed(tile);

        // Draw the tile

        commands.push(command_set_background_tiles(
            x / 8,
            y / 8,
            1,
            1,
            vec![tile_index],
        ));

        commands
    }

    // TODO add x, y params
    fn draw_image(&mut self, image: &DynamicImage) -> Vec<CommandData> {
        todo!()
    }

    fn draw_sprite(&mut self, id: usize, sprite: &Sprite, x: u8, y: u8) -> Vec<CommandData> {
        // Load the sprite's tile

        let (mut commands, tile_index) = self.load_tile_if_needed(&sprite.tile);

        // Draw the sprite

        // TODO if needed only
        commands.push(command_set_sprite_tile(id as u8, tile_index)); // TODO dangerous usize to u8 cast

        commands.push(command_set_sprite_position(id as u8, x, y));

        commands
    }
}

// Low-level commands

fn command_draw_text(text: &str, x: u32, y: u32) -> Vec<u8> {
    let mut data = vec![0u8, x as u8, y as u8, text.len() as u8];
    for char in text.chars() {
        data.push(char as u8);
    }
    data
}

fn command_load_tiles(
    is_background: bool,
    tile_index: u16,
    tile_count: u16,
    tiles_data: Vec<u8>,
) -> Vec<u8> {
    let mut data = vec![
        1u8,
        if is_background { 1 } else { 0 },
        ((tile_index & 0xFF00) >> 8) as u8,
        tile_index as u8,
        ((tile_count & 0xFF00) >> 8) as u8,
        tile_count as u8,
    ];

    for tile_byte in tiles_data.iter() {
        data.push(*tile_byte);
    }

    data
}

fn command_set_background_tiles(
    tile_x: u8,
    tile_y: u8,
    tiles_w: u8,
    tiles_h: u8,
    tiles_indices: Vec<u8>,
) -> Vec<u8> {
    let mut data = vec![2u8, tile_x, tile_y, tiles_w, tiles_h];

    for tile_index in tiles_indices.iter() {
        data.push(*tile_index);
    }

    data
}

fn command_set_sprite_tile(sprite_index: u8, tile_index: u8) -> Vec<u8> {
    vec![3u8, sprite_index, tile_index]
}

fn command_set_sprite_position(sprite_index: u8, x: u8, y: u8) -> Vec<u8> {
    vec![4u8, sprite_index, x, y]
}

// Helpers

fn tile_to_gb(tile: &Tile) -> Vec<u8> {
    let mut gb_tile = vec![0; 8 * 2];

    for (pixel_index, color) in tile.pixels.iter().enumerate() {
        // RGB to grayscale

        let luminance = 0.2126 * (color.r as f32 / 255.0)
            + 0.7152 * (color.g as f32 / 255.0)
            + 0.0722 * (color.b as f32 / 255.0);

        let grayscale = ((1.0 - luminance) * 4.0).clamp(0.0, 3.0) as u8; // TODO rounding errors, sometimes = 4

        // To GB tile format

        let pixel_y = pixel_index / 8;
        let pixel_x = pixel_index % 8;

        let row_offset = pixel_y * 2;

        gb_tile[row_offset] = gb_tile[row_offset] | ((grayscale & 0b10) >> 1) << (7 - pixel_x);
        gb_tile[row_offset + 1] = gb_tile[row_offset + 1] | (grayscale & 0b01) << (7 - pixel_x);
    }

    gb_tile
}
