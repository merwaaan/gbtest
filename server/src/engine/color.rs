#[derive(Debug, Clone, Copy, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub static BLACK: Color = Color::rgb(0x00, 0x00, 0x00);
pub static WHITE: Color = Color::rgb(0xFF, 0xFF, 0xFF);
pub static RED: Color = Color::rgb(0xFF, 0x00, 0x00);
pub static GREEN: Color = Color::rgb(0x00, 0xFF, 0x00);
pub static BLUE: Color = Color::rgb(0x00, 0x00, 0xFF);
