use std::sync::{Arc, Mutex};

use crate::types::Note;

#[derive(Clone)]
pub struct AppState {
    pub notes: Arc<Mutex<Vec<Note>>>,
}
