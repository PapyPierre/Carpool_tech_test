use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

pub type SharedState = Arc<Mutex<AppState>>;

pub struct AppState {
    pub friends: HashMap<String, HashSet<String>>,
    pub pending: HashMap<String, HashSet<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            friends: HashMap::new(),
            pending: HashMap::new(),
        }
    }
}
