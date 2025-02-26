use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use super::messages::Message;
use crate::types::MessageQueue;

pub fn send_message(message: Message, server_addr: &SocketAddr) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(server_addr)?;
    stream.write_all(&message.to_bytes())
}

pub fn wait_for_messages(message_queue: MessageQueue, bind_addr: SocketAddr) {
    tokio::spawn(async move {
        loop {
            let listener = TcpListener::bind(bind_addr).unwrap();

            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let addr = stream.peer_addr().unwrap();
                let mut bytes = Vec::new();

                match stream.read_to_end(&mut bytes) {
                    Ok(_) => {
                        let message = Message::from_bytes(&bytes).unwrap();
                        message_queue.lock().unwrap().push((message, addr));
                    }
                    Err(error) => println!("{}", error.to_string()),
                };
            }
        }
    });
}
