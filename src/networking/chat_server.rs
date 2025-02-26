use std::collections::HashMap;

use super::{messages::Message, sockets};
use crate::distance::calculate_distance;
use crate::{entities::client_data::ClientData, types::MessageQueue};

static MAX_DISTANCE: f32 = 100.0;

pub struct ChatServer {
    clients: HashMap<String, ClientData>,
    message_queue: MessageQueue,
    saved_messages: HashMap<String, Vec<(String, String)>>,
}

impl ChatServer {
    pub fn new(message_queue: MessageQueue) -> Self {
        ChatServer {
            clients: HashMap::new(),
            message_queue,
            saved_messages: HashMap::new(),
        }
    }

    pub fn send_message(&self, msg: Message, to: &str) -> Result<(), std::io::Error> {
        if let Some(client_addr) = &self.clients.get(to).map(|cd| cd.socket_addr) {
            sockets::send_message(msg, client_addr)?
        }
        Ok(())
    }

    pub fn handle_messages(&mut self) {
        if let Some((message, _)) = self.message_queue.lock().unwrap().pop() {
            println!("{:?}", message);

            match message {
                Message::NewClient(name, location, addr) => {
                    let client_data = ClientData::new(name.clone(), addr, location.0, location.1);
                    self.clients.insert(name, client_data);
                }
                Message::UpdatePosition(name, location) => {
                    let mut client_data = self.clients.remove(&name).unwrap();
                    client_data.current_loc = location;
                    self.clients.insert(name.clone(), client_data);

                    for (_, v) in self.clients.iter() {
                        if calculate_distance(v.current_loc, location) < MAX_DISTANCE {
                            let msg = Message::UpdatePosition(v.get_username(), v.current_loc);
                            self.send_message(msg, &name).unwrap();
                        }
                    }

                    self.send_cached_messages(name);
                }
                Message::ChatMessage(from, to, msg) => {
                    let from_location = self.clients.get(&from).unwrap().current_loc;
                    let to_location = self.clients.get(&to).unwrap().current_loc;

                    if calculate_distance(from_location, to_location) < MAX_DISTANCE {
                        let message = Message::ChatMessage(from, to.clone(), msg);
                        self.send_message(message, &to).unwrap();
                    } else {
                        println!("Out of range: Message saved to queue");
                        if let Some(queue) = self.saved_messages.get_mut(&from) {
                            queue.push((to, msg));
                        } else {
                            self.saved_messages.insert(to, vec![(from, msg)]);
                        }
                    }
                }
                Message::RefreshClients(username) => {
                    let user_loc = self.clients[&username].current_loc;

                    for (_, v) in self.clients.iter() {
                        if calculate_distance(v.current_loc, user_loc) < MAX_DISTANCE {
                            let msg = Message::UpdatePosition(v.get_username(), v.current_loc);
                            self.send_message(msg, &username).unwrap();
                        }
                    }
                }
            }
        }
    }

    fn send_cached_messages(&self, username: String) {
        if let Some(cached_messages) = self.saved_messages.get(&username) {
            let user_loc = self.clients[&username].current_loc;

            for (from, msg) in cached_messages {
                let target_loc = self.clients[from].current_loc;

                if calculate_distance(target_loc, user_loc) < MAX_DISTANCE {
                    let message = Message::ChatMessage(from.clone(), username.clone(), msg.clone());
                    self.send_message(message, &username).unwrap();
                }
            }
        }
    }
}
