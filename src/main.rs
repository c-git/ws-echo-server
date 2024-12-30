#![warn(unused_crate_dependencies)]
#![forbid(unsafe_code)]

use std::{net::TcpListener, thread::spawn};

#[shuttle_runtime::main]
async fn main() -> Result<DataStruct, shuttle_runtime::Error> {
    Ok(DataStruct)
}

pub struct DataStruct;

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for DataStruct {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let server = TcpListener::bind(dbg!(addr)).expect("failed to bind to address");
        for stream in server.incoming() {
            // TODO 2: Track number of active connections and setup to close out connections after timeout
            spawn(move || {
                // TODO 2: Log incoming IP
                let mut websocket = tungstenite::accept(stream.expect("TcpStream error"))
                    .expect("Hand shake failed");
                eprintln!("New client connected");
                while let Ok(msg) = websocket.read() {
                    // We do not want to send back ping/pong messages.
                    if msg.is_binary() || msg.is_text() {
                        if let Err(err) = websocket.send(msg) {
                            eprintln!("Error sending message: {err}");
                            break;
                        } else {
                            eprintln!("Responded.");
                        }
                    } else if msg.is_close() {
                        eprintln!("Connection closed.");
                    } else {
                        eprintln!("Unknown message received: {msg:?}");
                    }
                }
                eprintln!("Client left.");
            });
        }
        eprintln!("Server Shutdown Gracefully");
        Ok(())
    }
}
