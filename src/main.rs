use std::{
    sync::{Arc, Mutex},
    vec,
};

use axum::{
    Router,
    routing::{delete, get},
};

use crate::{
    handlers::{create_note, delete_note, get_all_notes, read_note, update_note},
    state::AppState,
};
mod handlers;
mod state;
mod types;
#[tokio::main]
async fn main() {
    let state = AppState {
        notes: Arc::new(Mutex::new(vec![])),
    };
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/notes", get(get_all_notes).post(create_note))
        .route("/notes/{note_id}", get(read_note).patch(update_note))
        .route("/notes/{note_id}", delete(delete_note))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
