use std::{thread, io, time::Duration, sync::mpsc};

use clap::Parser;

mod apps;
mod client;
mod commands;
mod server;

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: ServerCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum ServerCommand {  // TODO alias subcommands?
    Quit,
    Pos {
        client_id: u8,
        x: f32,
        y: f32
    }
}

fn main() {
    // Server thread

    let (sender, receiver) = mpsc::channel::<ServerCommand>();

    let server_thread = thread::spawn(move ||
    {
        let mut server = server::Server::new();

        server.start("127.0.0.1:3333");

        while server.is_running() {
            server.update();

            match receiver.try_recv() {
                Ok(command) => {
                    server.process_command(&command);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("disconnected");
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });
    
    // Terminal input
    
    loop {
        let mut stdin_input = String::new();
        io::stdin().read_line(&mut stdin_input).unwrap();
        stdin_input = stdin_input.trim().to_string();

        let stdin_items = stdin_input
            .split(" ")
            .map(|item| item.trim());

        // TODO ignore first item? (app name?)

        // Parse the command

        let cli = Args::parse_from(stdin_items);

        // Stop polling if quitting
        if matches!(cli.command, ServerCommand::Quit) {
            sender.send(cli.command);
            break;
        }

        sender.send(cli.command);
    }
    
    server_thread.join().unwrap();
}
