use std::sync::{Arc, Mutex};

use distance_chat::networking::chat_server::ChatServer;
use distance_chat::networking::sockets::wait_for_messages;

#[tokio::main]
async fn main() {
    let message_queue = Arc::new(Mutex::new(Vec::new()));
    let mut server = ChatServer::new(message_queue.clone());

    let thread_queue = message_queue.clone();
    tokio::spawn(async move {
        let socket = "127.0.0.1:10690".parse().unwrap();
        wait_for_messages(thread_queue, socket);
    });

    loop {
        server.handle_messages();
    }
}
