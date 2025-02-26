use std::collections::HashMap;
use std::net::SocketAddr;

use super::messages::Message;
use super::sockets;
use crate::types::{GeoLocation, MessageQueue};

#[derive(Clone)]
pub struct ChatClient {
    server_addr: SocketAddr,
    available_clients: HashMap<String, GeoLocation>,
    message_queue: MessageQueue,
}

impl ChatClient {
    pub fn new(server_addr: &str, message_queue: MessageQueue) -> Self {
        let server_addr = server_addr.parse().expect("Invalid server IP/Socket");

        ChatClient {
            server_addr,
            available_clients: HashMap::new(),
            message_queue,
        }
    }

    pub fn send_message(&self, msg: Message) -> Result<(), std::io::Error> {
        sockets::send_message(msg, &self.server_addr)
    }

    pub fn clear_clients(&mut self) {
        self.available_clients = HashMap::new();
    }

    pub fn handle_message(&mut self) -> Option<(String, String)> {
        if let Some((message, _)) = self.message_queue.lock().unwrap().pop() {
            return match message {
                Message::ChatMessage(from, _, msg) => Some((from, msg)),
                Message::UpdatePosition(name, position) => {
                    self.available_clients.insert(name.clone(), position);
                    Some((name, String::new()))
                }
                _ => panic!("Client received an invalid message type."),
            };
        }
        None
    }
}
