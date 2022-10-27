use crate::client::Client;
use crate::ServerCommand;
use crate::commands::ClientCommand;

pub trait App {
    fn update(&mut self, clients: &mut Vec<Client>);
    fn process_server_command(&mut self, command: &ServerCommand);
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
        self.x = self.x.wrapping_add(1);

        if self.x > 159 {
            self.x = 0;
            self.y = self.y.wrapping_add(1);
        }

        for client in clients {
            client.buffer_command(ClientCommand::DrawPoint(self.x, self.y));
        }
    }
    
    fn process_server_command(&mut self, command: &ServerCommand) {               
    }
}
