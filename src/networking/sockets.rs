use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use super::messages::{Message, BUFFER_SIZE};
use crate::types::MessageQueue;

pub fn send_message(message: Message, server_addr: &SocketAddr) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(server_addr)?;
    println!("Connected to Server");

    stream.write_all(&message.to_bytes())
}

pub fn wait_for_messages(message_queue: MessageQueue, bind_addr: SocketAddr) {
    tokio::spawn(async move {
        let (mut stream, addr) = TcpListener::bind(bind_addr).unwrap().accept().unwrap();

        let mut bytes: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        match stream.read_exact(&mut bytes) {
            Ok(_) => {
                let message = Message::from_bytes(&bytes).unwrap();
                message_queue.lock().unwrap().push((message, addr));
            }
            Err(error) => println!("{}", error.to_string()),
        };
    });
}
