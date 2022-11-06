#[derive(Debug, Clone)]
pub enum ClientCommand {
    ClearScreen,
    ClearRect(u8, u8, u8, u8),
    DrawPoint(u8, u8),
    DrawLine(u8, u8, u8, u8),
    DrawCircle(u8, u8, u8),
    PrintText(u8, u8, String),
    LoadTiles(bool, u16, u16, Vec<u8>),
    SetBackgroundTiles(u8, u8, u8, u8, Vec<u8>),
    SetSpriteTile(u8, u8),
    MoveSprite(u8, u8, u8),
}

impl ClientCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            ClientCommand::ClearScreen => vec![0],
            ClientCommand::ClearRect(x, y, w, h) => vec![1, *x, *y, *w, *h],
            ClientCommand::DrawPoint(x, y) => vec![2, *x, *y],
            ClientCommand::DrawLine(x1, y1, x2, y2) => vec![3, *x1, *y1, *x2, *y2],
            ClientCommand::DrawCircle(x, y, r) => vec![4, *x, *y, *r],
            ClientCommand::PrintText(x, y, text) => {
                let mut data = vec![5, *x, *y, text.len() as u8];
                for char in text.chars() {
                    data.push(char as u8);
                }
                // TODO add length? or \0?
                data
            }
            ClientCommand::LoadTiles(is_background, tile_index, tile_count, tile_data) => {
                let mut data = vec![
                    6,
                    if *is_background { 1 } else { 0 },
                    ((*tile_index & 0xFF00) >> 8) as u8,
                    *tile_index as u8,
                    ((*tile_count & 0xFF00) >> 8) as u8,
                    *tile_count as u8,
                ];

                for tile_byte in tile_data.iter() {
                    data.push(*tile_byte);
                }

                data
            }
            ClientCommand::SetBackgroundTiles(tile_x, tile_y, columns, rows, tile_indices) => {
                let mut data = vec![7, *tile_x, *tile_y, *columns, *rows];

                for tile_index in tile_indices.iter() {
                    data.push(*tile_index);
                }

                data
            }
            ClientCommand::SetSpriteTile(sprite_index, tile_index) => {
                vec![8, *sprite_index, *tile_index]
            }
            ClientCommand::MoveSprite(sprite_index, x, y) => {
                vec![9, *sprite_index, *x, *y]
            }
        }
    }
}
