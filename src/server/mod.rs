use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::connection::Connection;

pub struct Server {
    port: u32,
    banner: String,
    password: String,
    rooms: Arc<Mutex<HashMap<String, Room>>>,
}

pub struct Room {
    first: Connection,
    second: Connection,
    opened: Instant,
    full: bool,
}

impl Server {
    pub fn new(port: u32, banner: &str, password: &str) -> Self {
        Self {
            port,
            banner: banner.to_string(),
            password: password.to_string(),
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn run(port: u32, banner: &str, password: &str) {
        let server = Self::new(port, banner, password);
        server.start();
    }

    pub fn start(&self) {}

    pub fn delete_rooms(&self) {}

    pub fn listen(&self) {}
}

pub fn pipe() {}
