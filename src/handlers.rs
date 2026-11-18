use crate::{
    auth::{self, AuthUser},
    note_repo::NoteRepo,
    note_service::Note,
    state::AppState,
    types::{CreateNoteReq, CreateUser, LoginReq, LoginRes, UpdateNoteReq},
    user_repo::UserRepo,
    user_service::UserService,
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create_note(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(request): Json<CreateNoteReq>,
) -> (StatusCode, Json<Note>) {
    let note = Note::new(request, auth.user_id);
    let result = NoteRepo::save_note(&state.notes, note.clone()).await;
    let val = match result {
        Ok(_) => (StatusCode::CREATED, axum::Json(note.clone())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(note)),
    };
    return val;
}
#[axum::debug_handler]
pub async fn read_note(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<Note>, StatusCode> {
    let n = NoteRepo::read_note(&state.notes, note_id, auth.user_id).await;

    let note = match n {
        Ok(Some(n)) => n,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(Json(note))
}

#[axum::debug_handler]
pub async fn get_all_notes(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<Note>>, StatusCode> {
    let notes = NoteRepo::get_all_notes(&state.notes, auth.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(notes))
}
#[axum::debug_handler]
pub async fn update_note(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
    Json(update): Json<UpdateNoteReq>,
) -> Result<Json<Note>, StatusCode> {
    let now = Utc::now().to_rfc3339();

    let note = NoteRepo::update_note(&state.notes, note_id, auth.user_id, update, now)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    note.map(Json).ok_or(StatusCode::NOT_FOUND)
}

#[axum::debug_handler]
pub async fn delete_note(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> StatusCode {
    let result = NoteRepo::delete_note(&state.notes, note_id, auth.user_id).await;

    match result {
        Ok(r) if r.rows_affected() == 0 => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> StatusCode {
    let unwrapped_user = UserService::build_user(user);
    let user = match unwrapped_user {
        Ok(ok) => ok,
        Err(e) => return e,
    };
    let result = UserRepo::save_new_user(&state.users, user).await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => StatusCode::CONFLICT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[axum::debug_handler]
pub async fn login(
    state: State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<LoginRes>, StatusCode> {
    let user = match UserService::sign_in(state.users.clone(), req).await {
        Ok(user) => user,
        Err(code) => return Err(code),
    };
    let t = match UserService::create_token(user.id, &state.jwt_secret) {
        Ok(token) => token,
        Err(code) => return Err(code),
    };

    return Ok(Json(LoginRes { token: t }));
}
