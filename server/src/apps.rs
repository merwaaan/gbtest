pub mod bouncing_balls;
pub mod display_image;
pub mod fill_screens;
pub mod show_info;

use std::time::Duration;

use crate::client::Client;
use crate::ServerCommand;

pub trait App {
    fn update(&mut self, _dt: &Duration, _clients: &mut Vec<Client>) {}
    fn process_server_command(&mut self, _command: &ServerCommand) {}
}
