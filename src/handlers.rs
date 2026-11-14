use crate::note_repo::{self, NoteRepo};
use crate::note_service::{self, Note};
use crate::{
    state::AppState,
    types::{CreateNoteReq, UpdateNoteReq},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{self, Utc};
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create_note(
    State(state): State<AppState>,
    Json(request): Json<CreateNoteReq>,
) -> (StatusCode, Json<Note>) {
    let note = Note::new(request);
    let result = NoteRepo::save_note(&state.notes, note.clone()).await;
    let val = match result {
        Ok(ok) => (StatusCode::CREATED, axum::Json(note.clone())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(note)),
    };
    return val;
}
#[axum::debug_handler]
pub async fn read_note(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<Note>, StatusCode> {
    let n = NoteRepo::read_note(&state.notes, note_id).await;

    let note = match n {
        Ok(Some(n)) => n,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(Json(note))
}

#[axum::debug_handler]
pub async fn get_all_notes(State(state): State<AppState>) -> Result<Json<Vec<Note>>, StatusCode> {
    let notes = NoteRepo::get_all_notes(&state.notes)
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

    let note = NoteRepo::update_note(&state.notes, note_id, update, now)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    note.map(Json).ok_or(StatusCode::NOT_FOUND)
}

#[axum::debug_handler]
pub async fn delete_note(State(state): State<AppState>, Path(note_id): Path<Uuid>) -> StatusCode {
    let result = NoteRepo::delete_note(&state.notes, note_id).await;

    match result {
        Ok(r) if r.rows_affected() == 0 => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
