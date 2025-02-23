use std::collections::HashMap;

use distance_chat::entities::client::ClientData;

fn main() {
    let mut clients_by_username: HashMap<String, ClientData> = HashMap::new();

    let client = ClientData::new("john".to_string(), "1.1.1.1".to_string());
    clients_by_username.insert(client.get_username(), client);
}
