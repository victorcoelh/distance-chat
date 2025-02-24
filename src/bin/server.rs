use distance_chat::networking::messages::Message;

fn main() {
    let chat_msg =
        Message::ChatMessage("john".to_string(), "carter".to_string(), "hey!".to_string());

    let encrypted_msg = chat_msg.to_bytes();
    let decrypted_msg = Message::from_bytes(&encrypted_msg);

    println!("{:?}", decrypted_msg);
}
