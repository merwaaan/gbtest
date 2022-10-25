use crate::client::Client;
use crate::commands::Command;

pub trait App {
    fn update(&mut self, clients: &mut Vec<Client>);
}

//

pub struct TestApp {
    x: u8,
    y: u8,
}

impl TestApp {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl App for TestApp {
    fn update(&mut self, clients: &mut Vec<Client>) {
        println!("update");

        self.x = self.x.wrapping_add(1);

        if self.x > 159 {
            self.x = 0;
            self.y = self.y.wrapping_add(1);
        }

        for client in clients {
            client.buffer_command(Command::DrawPoint(self.x, self.y));
        }
    }
}
