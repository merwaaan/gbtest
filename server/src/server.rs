use crate::apps::display_image::DisplayImageApp;
use crate::ServerCommand;
use crate::{
    apps::{
        bouncing_balls::BouncingBallsApp, fill_screens::FillScreensApp, show_info::ShowInfoApp, App,
    },
    clients::client::Client,
    AppName,
};
use std::sync::mpsc::{Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{io, time::Instant};
use std::{net::TcpListener, vec::Vec};

pub struct Server {
    update_per_sec: u8,
    last_update_time: Instant,

    running: bool,

    connection_thread_handle: Option<JoinHandle<()>>,
    connection_thread_channel: Option<Sender<u8>>,
    clients: Arc<Mutex<Vec<Client>>>,

    app: Box<dyn App>,
}

impl Server {
    // TODO merge new/start
    pub fn new(update_per_sec: u8) -> Self {
        Server {
            update_per_sec,
            last_update_time: Instant::now(),
            running: false,
            connection_thread_handle: Option::None,
            connection_thread_channel: Option::None,
            clients: Arc::new(Mutex::new(Vec::new())),
            app: Box::new(DisplayImageApp::new()),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn wait_for_next_update(&self) {
        let update_interval = Duration::from_secs_f32(1.0f32 / self.update_per_sec as f32);

        let now = Instant::now();
        let since_last_update = now - self.last_update_time;

        let to_next_update = update_interval - since_last_update;

        thread::sleep(to_next_update);
    }

    pub fn start(&mut self, address: &str) {
        println!("Starting server");

        let a = String::from(address);

        let concurrent_clients = self.clients.clone();

        let (sender, receiver) = mpsc::channel();

        self.connection_thread_channel = Some(sender);

        self.connection_thread_handle = Some(thread::spawn(move || {
            let listener = TcpListener::bind(a).unwrap();
            listener.set_nonblocking(true).unwrap();
            // TODO stream blocking?

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New client: {}", stream.peer_addr().unwrap());
                        concurrent_clients
                            .lock()
                            .unwrap()
                            .push(Client::from_stream(stream));
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            match receiver.try_recv() {
                                Ok(_) => {
                                    return;
                                }
                                Err(TryRecvError::Disconnected) => {
                                    println!("Error: sender disconnected, {}", e);
                                    return;
                                }
                                _ => {}
                            }

                            continue;
                        } else {
                            println!("Error: {}", e);
                            return;
                        }
                    }
                }
            }
        }));

        self.running = true;
    }

    /*pub fn stop(&mut self) {
        println!("Stopping server");

        // TODO error handling

        self.connection_thread_channel
            .as_ref()
            .unwrap()
            .send(0)
            .unwrap();

        self.connection_thread_handle
            .take()
            .unwrap()
            .join()
            .unwrap();
    }*/

    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now - self.last_update_time;
        self.last_update_time = now;

        self.app.update(&dt, &mut self.clients.lock().unwrap());

        for client in self.clients.lock().unwrap().iter_mut() {
            client.send_commands();
        }
    }

    pub fn process_command(&mut self, command: &ServerCommand) {
        match command {
            ServerCommand::Quit => {
                println!("stopping server");
                self.running = false;
            }

            ServerCommand::App { app } => {
                println!("switching app to {:?}", app);

                self.app = match app {
                    AppName::Info => Box::new(ShowInfoApp::new()),
                    AppName::Fill => Box::new(FillScreensApp::new()),
                    AppName::Balls => Box::new(BouncingBallsApp::new()),
                };
            }

            _ => {}
        }

        // Forward to the clients

        for client in self.clients.lock().unwrap().iter_mut() {
            client.process_server_command(command);
        }

        // Forward to the app

        self.app.process_server_command(command);
    }
}
