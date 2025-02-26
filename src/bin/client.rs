use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use distance_chat::gui::state::ChatData;
use distance_chat::gui::update::subscription;
use distance_chat::gui::update::update;
use distance_chat::gui::view::main_view;
use distance_chat::networking::chat_client::ChatClient;
use distance_chat::networking::messages::Message;
use distance_chat::networking::sockets::wait_for_messages;
use iced::Task;
use tokio::time::sleep;

#[tokio::main]
pub async fn main() -> iced::Result {
    // get client name and socket
    let args: Vec<String> = env::args().collect();
    let name = args
        .get(1)
        .expect("Please insert a name for the client")
        .clone();
    let socket: SocketAddr = args
        .get(2)
        .expect("Please choose a socket to listen to")
        .parse()
        .expect("Couldn't read received socket address.");

    // initialize client
    let message_queue = Arc::new(Mutex::new(Vec::new()));
    let client = ChatClient::new("127.0.0.1:10690", message_queue.clone());
    let start_message = Message::NewClient(name.clone(), (0.0, 0.0), socket.clone());
    client
        .send_message(start_message)
        .expect("Could not connect to server.");

    // listen for incoming network messages
    let thread_queue = message_queue.clone();
    tokio::spawn(async move {
        wait_for_messages(thread_queue, socket);
    });

    // refresh clients every 2 minutes
    let thread_client = client.clone();
    let name_clone = name.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(2)).await;

            let message = Message::RefreshClients(name_clone.clone());
            thread_client.send_message(message).unwrap();
        }
    });

    // gui application
    iced::application("Distance Chat", update, main_view)
        .theme(|_| iced::Theme::CatppuccinMacchiato)
        .subscription(subscription)
        .run_with(move || (ChatData::new(client, name), Task::none()))
}
