use std::{
    sync::{Arc, Mutex},
    vec,
};

use crate::{
    handlers::{create_note, delete_note, get_all_notes, read_note, update_note},
    note_repo::NoteRepo,
    state::AppState,
};
use axum::{
    Router,
    routing::{delete, get},
};
use sqlx::SqlitePool;
mod handlers;
mod note_repo;
mod note_service;
mod state;
mod types;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqlitePool::connect(&database_url).await;

    let pool = match connection {
        Ok(sql) => {
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS notes (
    id            BLOB PRIMARY KEY,
    title         TEXT NOT NULL,
    subtitle      TEXT NOT NULL,
    texts         TEXT NOT NULL,
    footnote      TEXT NOT NULL,
    author        TEXT NOT NULL,
    category      TEXT NOT NULL,
    created_at    TEXT NOT NULL,
    last_modified TEXT NOT NULL
) STRICT
",
            )
            .execute(&sql)
            .await
            .expect("failed to create users table");
            sql
        }
        Err(e) => panic!("failed to connect to database: {e}"),
    };

    let state = AppState {
        notes: NoteRepo::new(pool),
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
