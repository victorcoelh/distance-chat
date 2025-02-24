use crate::types::GeoLocation;

pub const BUFFER_SIZE: usize = 512;

#[derive(Debug)]
pub enum Message {
    NewClient(String, GeoLocation),
    UpdatePosition(String, GeoLocation),
    ChatMessage(String, String, String),
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = match self {
            Self::NewClient(username, pos) => {
                format!("{}\n{}\n{}\n{}\n", "0", username, pos.0, pos.1)
            }
            Self::UpdatePosition(username, pos) => {
                format!("{}\n{}\n{}\n{}\n", "1", username, pos.0, pos.1)
            }
            Self::ChatMessage(from, to, text) => format!("{}\n{}\n{}\n{}\n", "2", from, to, text),
        }
        .into_bytes();

        bytes.resize(BUFFER_SIZE, 0); // pads the buffer with NULL characters
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut bytes_iter = bytes.iter();
        let message_id = bytes_iter.next().expect("Empty message received.");

        match message_id {
            b'0' => {
                let username = Message::get_string_from_iter(&mut bytes_iter);
                let lat = Message::get_string_from_iter(&mut bytes_iter)
                    .parse()
                    .unwrap();
                let long = Message::get_string_from_iter(&mut bytes_iter)
                    .parse()
                    .unwrap();

                Ok(Message::NewClient(username, (lat, long)))
            }
            b'1' => {
                let username = Message::get_string_from_iter(&mut bytes_iter);
                let lat = Message::get_string_from_iter(&mut bytes_iter)
                    .parse()
                    .unwrap();
                let long = Message::get_string_from_iter(&mut bytes_iter)
                    .parse()
                    .unwrap();

                Ok(Message::UpdatePosition(username, (lat, long)))
            }
            b'2' => {
                let from = Message::get_string_from_iter(&mut bytes_iter);
                let to = Message::get_string_from_iter(&mut bytes_iter);
                let text = Message::get_string_from_iter(&mut bytes_iter);

                Ok(Message::ChatMessage(from, to, text))
            }
            _ => panic!("Invalid message type received."),
        }
    }

    fn get_string_from_iter<'a, T>(bytes_iter: &mut T) -> String
    where
        T: Iterator<Item = &'a u8>,
    {
        String::from_utf8(
            bytes_iter
                .skip_while(|char| **char == b'\n')
                .take_while(|char| **char != b'\n')
                .map(|char| *char)
                .collect(),
        )
        .unwrap()
    }
}
