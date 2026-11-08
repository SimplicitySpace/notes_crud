use std::{sync::Arc, usize};

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

pub struct Handlers {}

#[axum::debug_handler]
pub async fn create_note(
    state: State<AppState>,
    request: Json<CreateNoteReq>,
) -> (StatusCode, Json<Note>) {
    let now = Utc::now();
    let unboxed_state = state.0;
    let notes = unboxed_state.notes;
    let req = request.0;
    let mut lock: std::sync::MutexGuard<'_, Vec<Note>> = notes.lock().unwrap();
    let lenghth = lock.len();
    let curr_id: usize = lenghth + 1;
    let note = Note {
        created_at: now.to_string(),
        last_modified: now.to_string(),
        title: req.title,
        subtitle: req.subtitle,
        texts: req.texts,
        footnote: req.footnote,
        author: req.author,
        category: req.category,
        id: curr_id,
    };
    lock.push(note.clone());
    (StatusCode::CREATED, axum::Json(note))
}
#[axum::debug_handler]
pub async fn read_note(
    state: State<AppState>,
    Path(note_id): Path<usize>,
) -> Result<Json<Note>, StatusCode> {
    let unboxed_state = state.0;
    let notes = unboxed_state.notes;
    let lock: std::sync::MutexGuard<'_, Vec<Note>> = notes.lock().unwrap();

    for n in lock.iter() {
        if n.id == note_id {
            return Ok(Json(n.clone()));
        }
    }
    Err(StatusCode::NOT_FOUND)
}

#[axum::debug_handler]
pub async fn get_all_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    let notes = state.notes.lock().unwrap();
    Json(notes.clone())
}
#[axum::debug_handler]
pub async fn update_note(
    State(state): State<AppState>,
    Path(note_id): Path<usize>,
    Json(update): Json<UpdateNoteReq>,
) -> Result<Json<Note>, StatusCode> {
    let mut notes = state.notes.lock().unwrap();

    let existing = notes
        .iter_mut()
        .find(|n| n.id == note_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = update.title {
        existing.title = title;
    }
    if let Some(subtitle) = update.subtitle {
        existing.subtitle = subtitle;
    }
    if let Some(texts) = update.texts {
        existing.texts = texts;
    }
    if let Some(footnote) = update.footnote {
        existing.footnote = footnote;
    }
    if let Some(author) = update.author {
        existing.author = author;
    }
    if let Some(category) = update.category {
        existing.category = category;
    }
    existing.last_modified = Utc::now().to_string();

    Ok(Json(existing.clone()))
}
#[axum::debug_handler]
pub async fn delete_note(State(state): State<AppState>, Path(note_id): Path<usize>) -> StatusCode {
    let mut notes = state.notes.lock().unwrap();

    let index = notes
        .iter()
        .position(|n| n.id == note_id)
        .ok_or(StatusCode::NOT_FOUND);

    match index {
        Ok(i) => {
            notes.remove(i);
            StatusCode::NO_CONTENT
        }
        Err(e) => e,
    }
}
