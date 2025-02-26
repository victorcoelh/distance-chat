use iced::{
    futures::{SinkExt, Stream},
    stream, Subscription,
};

use crate::networking::{chat_client::ChatClient, messages::Message};

use super::state::{ChatData, UIMessage};

pub fn update(client_data: &mut ChatData, message: UIMessage) {
    match message {
        UIMessage::AddMessage(name, text) => client_data.add_message(name, text),
        UIMessage::ClearClients => client_data.clear_clients(),
        UIMessage::AddClient(name) => {
            if !client_data.current_clients.contains(&name) {
                client_data.update_clients(name)
            }
        }
        UIMessage::PositionBoxChanged(content) => client_data.position_text = content,
        UIMessage::ChatBoxChanged(content) => client_data.chatbox_text = content,
        UIMessage::FocusOn(name) => client_data.current_target = Some(name),
        UIMessage::UpdatePosition => {
            let new_pos: Vec<&str> = client_data.position_text.split(",").collect();

            if new_pos.len() != 2 {
                return ();
            }

            let lat: f32 = new_pos[0]
                .trim()
                .parse()
                .expect("Couldn't parse string to float");
            let long: f32 = new_pos[1]
                .trim()
                .parse()
                .expect("Couldn't parse string to float");

            let message = Message::UpdatePosition(client_data.name.clone(), (lat, long));
            client_data.chat_client.send_message(message).unwrap();
            client_data.clear_clients();
        }
        UIMessage::SendMsg => {
            let text_msg = format!("{}: {}", client_data.name, client_data.chatbox_text);

            let message = Message::ChatMessage(
                client_data.name.clone(),
                client_data.current_target.clone().unwrap(),
                text_msg.clone(),
            );

            client_data.chat_client.send_message(message).unwrap();
            client_data
                .chats
                .get_mut(&client_data.current_target.clone().unwrap())
                .unwrap()
                .push(text_msg);

            client_data.chatbox_text = String::new();
        }
    }
}

pub fn subscription(client_data: &ChatData) -> Subscription<UIMessage> {
    Subscription::run_with_id(
        "Chat Subscription",
        get_messages_from_channel(client_data.chat_client.clone()),
    )
}

pub fn get_messages_from_channel(client: ChatClient) -> impl Stream<Item = UIMessage> {
    stream::channel(10000, |mut output| async move {
        loop {
            let mut client_clone = client.clone();

            let (name, text) = tokio::task::spawn_blocking(move || loop {
                if let Some((name, text)) = client_clone.handle_message() {
                    return (name, text);
                }
            })
            .await
            .unwrap();

            if text.is_empty() {
                output.send(UIMessage::AddClient(name)).await.unwrap();
            } else {
                output
                    .send(UIMessage::AddMessage(name, text))
                    .await
                    .unwrap();
            }
        }
    })
}
