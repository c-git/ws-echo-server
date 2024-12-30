#![warn(unused_crate_dependencies)]
#![forbid(unsafe_code)]

use std::{net::TcpListener, thread::spawn};

fn main() {
    let bind_addr = "127.0.0.1:9001";
    let server = TcpListener::bind(bind_addr).unwrap();
    eprintln!("Listening on: ws://{bind_addr}");
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
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
}
