use std::collections::HashSet;
use std::time::Duration;

use crate::client::Client;
use crate::{apps::App, commands::ClientCommand};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use parry2d::bounding_volume::{BoundingVolume, AABB};

pub struct DisplayImageApp {
    area: AABB,
    image: DynamicImage,
    known_client_ids: HashSet<u8>,
}

impl DisplayImageApp {
    pub fn new() -> Self {
        let image = image::open("test.png").unwrap();

        println!("Loaded image {:?}", image.dimensions());

        Self {
            area: AABB::new_invalid(),
            image,
            known_client_ids: HashSet::new(),
        }
    }
}

impl App for DisplayImageApp {
    fn update(&mut self, _dt: &Duration, clients: &mut Vec<Client>) {
        // Compute the current bounding box

        let mut new_aabb = AABB::new_invalid();

        for client in clients.iter_mut() {
            new_aabb.merge(&client.screen().bounding_box());
        }

        if new_aabb != self.area {
            self.area = new_aabb;

            // TODO correct sub images
        }

        // Handle new clients

        for client in clients.iter_mut() {
            if !self.known_client_ids.contains(&client.id()) {
                self.known_client_ids.insert(client.id());

                //

                let screen_x_normalized =
                    (client.screen().pos.x - self.area.mins.x) / self.area.extents().x;
                let screen_y_normalized =
                    (client.screen().pos.y - self.area.mins.y) / self.area.extents().y;

                let screen_w_normalized = client.screen().size.x / self.area.extents().x;
                let screen_h_normalized = client.screen().size.y / self.area.extents().y;

                let area_ratio = self.area.extents().x / self.area.extents().y;
                let image_ratio =
                    self.image.dimensions().0 as f32 / self.image.dimensions().1 as f32;
                let relative_ratio = area_ratio / image_ratio;

                println!(
                    "area {}, image {}, rel {}",
                    area_ratio, image_ratio, relative_ratio
                );

                let image_x_normalized = if area_ratio > image_ratio {
                    screen_x_normalized
                } else {
                    screen_x_normalized * relative_ratio
                };

                let image_y_normalized = if area_ratio > image_ratio {
                    screen_y_normalized * relative_ratio
                } else {
                    screen_y_normalized
                };

                let image_w_normalized = if area_ratio > image_ratio {
                    screen_w_normalized
                } else {
                    screen_w_normalized * relative_ratio
                };

                let image_h_normalized = if area_ratio > image_ratio {
                    screen_h_normalized * relative_ratio
                } else {
                    screen_h_normalized
                };

                //

                let image_x = (image_x_normalized * self.image.dimensions().0 as f32) as u32;
                let image_y = (image_y_normalized * self.image.dimensions().1 as f32) as u32;
                let image_w = (image_w_normalized * self.image.dimensions().0 as f32) as u32;
                let image_h = (image_h_normalized * self.image.dimensions().1 as f32) as u32;

                let cropped_image = self.image.crop(image_x, image_y, image_w, image_h);

                println!(
                    "New client: pos {:?}, size {:?}, {:?} {:?} {:?} {:?}",
                    client.screen().pos,
                    client.screen().size,
                    image_x,
                    image_y,
                    image_w,
                    image_h
                );

                fill_screen_with_image(client, &cropped_image);
            }
        }
    }
}

fn fill_screen_with_image(client: &mut Client, image: &DynamicImage) {
    let resized_image = image.resize(
        client.screen().res.x as u32,
        client.screen().res.y as u32,
        FilterType::Nearest,
    );

    // Convert the image to tiles

    let mut tiles_data = [0u8; 20 * 18 * 16];

    for screen_y in 0..client.screen().res.y {
        for screen_x in 0..client.screen().res.x {
            // Pixel to grayscale

            let pixel = &resized_image.get_pixel(screen_x as u32, screen_y as u32);

            let luminance = 0.2126 * (pixel[0] as f32 / 255.0)
                + 0.7152 * (pixel[1] as f32 / 255.0)
                + 0.0722 * (pixel[2] as f32 / 255.0);

            let grayscale = ((1.0 - luminance) * 4.0).clamp(0.0, 3.0) as u8; // TODO rounding errors, sometimes = 4

            if screen_x < 8 && screen_y < 8 {
                println!("{screen_x} {screen_y} {:?} {luminance} {grayscale} ", pixel);
            }

            // To GB tile format

            let tile_y = screen_y / 8;
            let tile_x = screen_x / 8;

            let tile_pixel_y = screen_y % 8;
            let tile_pixel_x = screen_x % 8;

            let tile_row_offset = (tile_y * 20 + tile_x) * 16 + tile_pixel_y * 2;

            tiles_data[tile_row_offset] =
                tiles_data[tile_row_offset] | (grayscale & 0b10) << (7 - tile_pixel_x);

            tiles_data[tile_row_offset + 1] =
                tiles_data[tile_row_offset + 1] | (grayscale & 0b01) << (7 - tile_pixel_x);
        }
    }

    let actual_tile_count: u8 = 128; // TODO cannot store whole bg of 20x18 for now!

    client.buffer_command(ClientCommand::LoadTiles(
        true,
        0,
        actual_tile_count as u16,
        tiles_data[0..actual_tile_count as usize * 16].to_vec(),
    ));

    // Place the tiles on the screen

    let tiles_indices: Vec<u8> = (0..actual_tile_count).collect();

    client.buffer_command(ClientCommand::SetBackgroundTiles(
        0,
        0,
        20,
        18,
        tiles_indices,
    ));
}
