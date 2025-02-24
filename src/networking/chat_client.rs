use std::net::SocketAddr;

use super::messages::Message;
use super::sockets;
use crate::types::MessageQueue;

pub struct ChatClient {
    server_addr: SocketAddr,
    message_queue: MessageQueue,
}

impl ChatClient {
    pub fn new(server_addr: &str, message_queue: MessageQueue) -> Self {
        let server_addr = server_addr.parse().expect("Invalid server IP/Socket");

        ChatClient {
            server_addr,
            message_queue,
        }
    }

    pub fn send_message(&self, msg: Message) -> Result<(), std::io::Error> {
        sockets::send_message(msg, &self.server_addr)
    }
}
