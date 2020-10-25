use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;

extern crate env_logger;
extern crate log;

use log::info;
use env_logger::Env;

fn main() {
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    info!("Binding tcp listener");
    let server = TcpListener::bind("0.0.0.0:9001").unwrap();
    for stream in server.incoming() {
        info!("Stream recieved");
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    let response = "Hello client";
                    info!("responding \"{}\" with \"{}\"", msg, response);
                    websocket
                        .write_message(Message::Text(response.into()))
                        .unwrap();
                }
            }
        });
    }
}
