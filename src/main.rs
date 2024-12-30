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
                let stream = stream.expect("TcpStream error");
                let peer_addr = format!("{:?}", stream.peer_addr());
                let mut websocket = tungstenite::accept(stream).expect("Hand shake failed");
                println!("{peer_addr} - established a new connection");
                while let Ok(msg) = websocket.read() {
                    // We do not want to send back ping/pong messages.
                    println!("{peer_addr} - sent: {msg:?}");
                    if msg.is_binary() || msg.is_text() {
                        if let Err(err) = websocket.send(msg) {
                            eprintln!("{peer_addr} - Error sending message: {err}");
                            break;
                        } else {
                            println!("{peer_addr} - was sent back a copy of their message");
                        }
                    } else if msg.is_close() {
                        println!("{peer_addr} - sent a close connection message");
                    } else {
                        eprintln!("{peer_addr} - sent an unknown message: {msg:?}");
                    }
                }
                println!("{peer_addr} - disconnected");
            });
        }
        eprintln!("Server Shutdown Gracefully");
        Ok(())
    }
}
