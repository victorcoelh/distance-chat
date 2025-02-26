use std::collections::HashMap;

use crate::{networking::chat_client::ChatClient, types::GeoLocation};

#[derive(Debug, Clone)]
pub enum UIMessage {
    ClearClients,
    AddClient(String),
    AddMessage(String, String),
    PositionBoxChanged(String),
    ChatBoxChanged(String),
    FocusOn(String),
    UpdatePosition,
    SendMsg,
}

pub struct ChatData {
    pub chat_client: ChatClient,
    pub name: String,
    pub location: GeoLocation,
    pub current_target: Option<String>,
    pub position_text: String,
    pub chatbox_text: String,
    pub current_clients: Vec<String>,
    pub chats: HashMap<String, Vec<String>>,
}

impl ChatData {
    pub fn new(chat_client: ChatClient, name: String) -> Self {
        ChatData {
            chat_client,
            name,
            location: (0.0, 0.0),
            current_target: None,
            position_text: String::new(),
            chatbox_text: String::new(),
            current_clients: Vec::new(),
            chats: HashMap::new(),
        }
    }

    pub fn clear_clients(&mut self) {
        self.current_clients = Vec::new();
    }

    pub fn update_clients(&mut self, new_client: String) {
        self.current_clients.push(new_client.clone());
        if !self.chats.contains_key(&new_client) {
            self.chats.insert(new_client, Vec::new());
        }
    }

    pub fn add_message(&mut self, user: String, text: String) {
        self.chats.get_mut(&user).unwrap().push(text);
    }
}
