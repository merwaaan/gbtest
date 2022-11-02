use crate::client::Client;

#[derive(Debug, Clone)]
pub enum ClientCommand {
    ClearScreen,
    ClearRect(u8, u8, u8, u8),
    DrawPoint(u8, u8),
    DrawLine(u8, u8, u8, u8),
    DrawCircle(u8, u8, u8),
    PrintText(u8, u8, String),
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
        }
    }
}
