use crate::{
    state::AppState,
    types::{CreateNoteReq, Note, UpdateNoteReq},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{self, Utc};
use uuid::Uuid;

pub struct Handlers {}

#[axum::debug_handler]
pub async fn create_note(
    State(state): State<AppState>,
    Json(request): Json<CreateNoteReq>,
) -> Result<(StatusCode, Json<Note>), StatusCode> {
    let now = Utc::now();

    let id = Uuid::now_v7();
    let note = Note {
        created_at: now.to_string(),
        last_modified: now.to_string(),
        title: request.title,
        subtitle: request.subtitle,
        texts: request.texts,
        footnote: request.footnote,
        author: request.author,
        category: request.category,
        id: id,
    };
    sqlx::query(
        "INSERT INTO notes 
            (id, title, subtitle, texts, footnote, author, category, created_at, last_modified)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&note.id)
    .bind(&note.title)
    .bind(&note.subtitle)
    .bind(&note.texts)
    .bind(&note.footnote)
    .bind(&note.author)
    .bind(&note.category)
    .bind(&note.created_at)
    .bind(&note.last_modified)
    .execute(&state.notes)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, axum::Json(note)))
}
#[axum::debug_handler]
pub async fn read_note(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<Note>, StatusCode> {
    let u_note: Result<Option<Note>, sqlx::Error> =
        sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ?")
            .bind(note_id)
            .fetch_optional(&state.notes)
            .await;

    let note = match u_note {
        Ok(Some(n)) => n,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(Json(note))
}

#[axum::debug_handler]
pub async fn get_all_notes(State(state): State<AppState>) -> Result<Json<Vec<Note>>, StatusCode> {
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes ORDER BY id")
        .fetch_all(&state.notes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(notes))
}
#[axum::debug_handler]
pub async fn update_note(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
    Json(update): Json<UpdateNoteReq>,
) -> Result<Json<Note>, StatusCode> {
    let now = Utc::now().to_rfc3339();

    let note = sqlx::query_as::<_, Note>(
        "UPDATE notes SET
            title         = COALESCE(?, title),
            subtitle      = COALESCE(?, subtitle),
            texts         = COALESCE(?, texts),
            footnote      = COALESCE(?, footnote),
            author        = COALESCE(?, author),
            category      = COALESCE(?, category),
            last_modified = ?
         WHERE id = ?
         RETURNING *",
    )
    .bind(update.title) // Option<String>
    .bind(update.subtitle)
    .bind(update.texts)
    .bind(update.footnote)
    .bind(update.author)
    .bind(update.category)
    .bind(now) // last_modified
    .bind(note_id) // WHERE id = ?
    .fetch_optional(&state.notes)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    note.map(Json).ok_or(StatusCode::NOT_FOUND)
}

#[axum::debug_handler]
pub async fn delete_note(State(state): State<AppState>, Path(note_id): Path<Uuid>) -> StatusCode {
    let result = sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(note_id)
        .execute(&state.notes)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
