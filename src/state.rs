use crate::{note_repo::NoteRepo, user_repo::UserRepo};
#[derive(Clone)]
pub struct AppState {
    pub notes: NoteRepo,
    pub users: UserRepo,
    pub jwt_secret: String,
}
