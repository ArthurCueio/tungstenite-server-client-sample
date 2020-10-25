use tungstenite::{connect, Message};
use url::Url;

extern crate env_logger;
extern crate log;

use log::info;
use env_logger::Env;

use std::{thread, time};

fn main() {
    env_logger::init_from_env(Env::new().default_filter_or("info"));
    info!("Client started");

    let sleep_time = time::Duration::from_secs(1);

    let mut socket;
    loop {
        let connection = connect(Url::parse("ws://192.168.0.103:9001").unwrap());
        match connection {
            Ok(result) => {
                info!("Connected");
                socket = result.0;
                break;
            }
            Err(error) => {
                info!("Couldn't connect to server: {}", error);
                thread::sleep(sleep_time);
                info!("Retrying...");
            }
        }
    }

    let msg = "Hello server";
    info!("Sending message {}", msg);
    socket.write_message(Message::Text(msg.into())).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        info!("Received: {}", msg);
    }
    //socket.close(None);
}
