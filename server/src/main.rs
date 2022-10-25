use std::{thread, time::Duration};

mod app;
mod client;
mod commands;
mod server;

fn main() {
    let mut server = server::Server::new();

    server.start("127.0.0.1:3333");

    loop {
        server.update();
        thread::sleep(Duration::from_millis(1000));
    }

    server.stop();
}
