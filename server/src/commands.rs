use crate::client::Client;

#[derive(Debug, Clone)]
pub enum ClientCommand {
    ClearScreen,
    DrawPoint(u8, u8),
    DrawLine(u8, u8, u8, u8),
    DrawCircle(u8, u8, u8),
    PrintText(String),
}

impl ClientCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            ClientCommand::ClearScreen => vec![0],
            ClientCommand::DrawPoint(x, y) => vec![1, *x, *y],
            ClientCommand::DrawLine(x1, y1, x2, y2) => vec![2, *x1, *y1, *x2, *y2],
            ClientCommand::DrawCircle(x, y, r) => vec![3, *x, *y, *r],
            ClientCommand::PrintText(text) => {
                let mut data = vec![4];
                for char in text.chars() {
                    data.push(char as u8);
                }
                // TODO add length? or \0?
                data
            }
        }
    }
}
