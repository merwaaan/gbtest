pub mod bouncing_balls;
pub mod fill_screens;
pub mod show_info;

use std::time::Duration;

use crate::client::Client;
use crate::ServerCommand;
use crate::commands::ClientCommand;

pub trait App {
    fn update(&mut self, dt: &Duration, clients: &mut Vec<Client>) {}
    fn process_server_command(&mut self, command: &ServerCommand) {}
}    
