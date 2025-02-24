use std::collections::HashMap;

use super::{messages::Message, sockets};
use crate::{entities::client_data::ClientData, types::MessageQueue};

pub struct ChatServer {
    clients: HashMap<String, ClientData>,
    message_queue: MessageQueue,
}

impl ChatServer {
    pub fn new(message_queue: MessageQueue) -> Self {
        ChatServer {
            clients: HashMap::new(),
            message_queue,
        }
    }

    pub fn send_message(&self, msg: Message, to: &str) -> Result<(), std::io::Error> {
        let client_addr = &self.clients.get(to).unwrap().socket_addr;
        sockets::send_message(msg, client_addr)
    }

    pub fn handle_messages(&mut self) {
        if let Some((message, addr)) = self.message_queue.lock().unwrap().pop() {
            match message {
                Message::NewClient(name, location) => {
                    let client_data = ClientData::new(name.clone(), addr, location.0, location.1);
                    self.clients.insert(name, client_data);
                }
                Message::UpdatePosition(name, location) => {
                    let mut client_data = self.clients.remove(&name).unwrap();
                    client_data.current_loc = location;
                    self.clients.insert(name, client_data);
                }
                Message::ChatMessage(from, to, msg) => {
                    let message = Message::ChatMessage(from, to.clone(), msg);
                    self.send_message(message, &to).unwrap();
                }
            }
        }
    }
}
