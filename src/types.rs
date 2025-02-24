use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::networking::messages::Message;

pub type GeoLocation = (f32, f32);
pub type MessageQueue = Arc<Mutex<Vec<(Message, SocketAddr)>>>;
