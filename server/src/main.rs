use std::{io, sync::mpsc, thread, time::Duration};

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
pub enum ServerCommand {
    // TODO alias subcommands?
    Quit,
    Pos { client_id: u8, x: f32, y: f32 },
    App { app: AppName },
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum AppName {
    Info,
    Fill,
    Balls,
}

fn main() {
    // Server thread

    let (sender, receiver) = mpsc::channel::<ServerCommand>();

    let server_thread = thread::spawn(move || {
        let mut server = server::Server::new(10u8);

        server.start("127.0.0.1:3333");

        while server.is_running() {
            match receiver.try_recv() {
                Ok(command) => {
                    server.process_command(&command);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("disconnected");
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }

            server.update();
            server.wait_for_next_update();
        }
    });

    // Terminal input

    loop {
        let mut stdin_input = String::new();
        io::stdin().read_line(&mut stdin_input).unwrap();
        stdin_input = stdin_input.trim().to_string();

        let stdin_items = stdin_input.split(" ").map(|item| item.trim());

        // TODO ignore first item? (app name?)

        // Parse the command

        match Args::try_parse_from(stdin_items) {
            Ok(cli) => {
                // Stop polling if quitting
                if matches!(cli.command, ServerCommand::Quit) {
                    sender.send(cli.command).unwrap();
                    break;
                }

                sender.send(cli.command).unwrap();
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    server_thread.join().unwrap();
}
