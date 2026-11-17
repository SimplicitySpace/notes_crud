use crate::{
    handlers::{
        create_note, create_user, delete_note, get_all_notes, login, read_note, update_note,
    },
    note_repo::NoteRepo,
    state::AppState,
    user_repo::UserRepo,
};
use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::{delete, get, post},
};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
mod handlers;
mod note_repo;
mod note_service;
mod state;
mod types;
mod user_repo;
mod user_service;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DB_URL").expect("DB_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let unwrapped_db = SqlitePool::connect(&database_url).await;
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);
    let db = match unwrapped_db {
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
    last_modified TEXT NOT NULL,
       user_id BLOB NOT NULL REFERENCES users(id) ON DELETE CASCADE
) STRICT
",
            )
            .execute(&sql)
            .await
            .expect("failed to create users table");

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS users (
    id              BLOB PRIMARY KEY,
    user_name       TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    first_name      TEXT NOT NULL,
    last_name       TEXT NOT NULL,
    sex             TEXT NOT NULL,
    nationality     TEXT NOT NULL,
    age             INTEGER NOT NULL,
    token           BLOB
) STRICT

",
            )
            .execute(&sql)
            .await
            .expect("failed to DB");
            sql
        }
        Err(e) => panic!("failed to connect to database: {e}"),
    };

    let state = AppState {
        notes: NoteRepo::new(db.clone()),
        users: UserRepo::new(db),
        jwt_secret,
    };
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/notes", get(get_all_notes).post(create_note))
        .route("/notes/{note_id}", get(read_note).patch(update_note))
        .route("/notes/{note_id}", delete(delete_note))
        .route("/create_user", post(create_user))
        .route("/login", post(login))
        .with_state(state)
        .layer(cors);
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => l,
        Err(e) => panic!("could not bind to port 3000 (is the server already running?): {e}"),
    };
    axum::serve(listener, app).await.unwrap();
}
