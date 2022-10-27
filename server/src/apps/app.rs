use crate::client::Client;
use crate::ServerCommand;
use crate::commands::ClientCommand;

pub trait App {
    fn update(&mut self, clients: &mut Vec<Client>);
    fn process_server_command(&mut self, command: &ServerCommand);
}
