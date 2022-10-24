use crate::client::Client;
use crate::commands::Command;

pub trait App {
    fn update(&mut self, clients: &mut Vec<Client>);
}

//

pub struct TestApp {
    n: u8,
}

impl TestApp {
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl App for TestApp {
    fn update(&mut self, clients: &mut Vec<Client>) {
        println!("update");

        self.n += 1;

        for client in clients {
            client.buffer_command(Command::DrawCircle(self.n, 10, 3));
        }
    }
}
