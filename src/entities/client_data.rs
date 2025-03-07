use std::net::SocketAddr;

use crate::types::GeoLocation;

pub struct ClientData {
    pub username: String,
    pub socket_addr: SocketAddr,
    pub current_loc: GeoLocation,
}

impl ClientData {
    pub fn new(username: String, socket_addr: SocketAddr, lat: f32, long: f32) -> Self {
        ClientData {
            username,
            socket_addr,
            current_loc: (lat, long),
        }
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }
}
