use crate::app::TestApp;
use crate::{app::App, client::Client};
use std::io;
use std::sync::mpsc::{Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::{net::TcpListener, vec::Vec};

pub struct Server {
    connection_thread_handle: Option<JoinHandle<()>>,
    connection_thread_channel: Option<Sender<u8>>,
    clients: Arc<Mutex<Vec<Client>>>,
    game: TestApp,
}

impl Server {
    // TODO merge new/start
    pub fn new() -> Self {
        Server {
            connection_thread_handle: Option::None,
            connection_thread_channel: Option::None,
            clients: Arc::new(Mutex::new(Vec::new())),
            game: TestApp::new(),
        }
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
                        concurrent_clients.lock().unwrap().push(Client::new(stream));
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            match receiver.try_recv() {
                                Ok(_) => {
                                    return;
                                }
                                Err(TryRecvError::Disconnected) => {
                                    println!("Error: sender disconnected, {}", e);
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
    }

    pub fn stop(&mut self) {
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
    }

    pub fn update(&mut self) {
        self.game.update(&mut self.clients.lock().unwrap());

        for client in self.clients.lock().unwrap().iter_mut() {
            client.send_commands();
        }
    }
}
