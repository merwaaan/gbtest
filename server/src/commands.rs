#[derive(Debug, Clone, Copy)]
pub enum ClientCommand {
    //PrintText(String),
    DrawPoint(u8, u8),
    DrawLine(u8, u8, u8, u8),
    DrawCircle(u8, u8, u8),
}

impl ClientCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            /*Command::PrintText(text) => vec![0]
            .into_iter()
            .chain(text.into_bytes().into_iter())
            .collect(),*/
            ClientCommand::DrawPoint(x, y) => vec![1, *x, *y],
            ClientCommand::DrawLine(x1, y1, x2, y2) => vec![2, *x1, *y1, *x2, *y2],
            ClientCommand::DrawCircle(x, y, r) => vec![3, *x, *y, *r],
        }
    }
}
