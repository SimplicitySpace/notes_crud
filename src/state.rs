
use crate::note_repo::NoteRepo;
#[derive(Clone)]
pub struct AppState {
    pub notes: NoteRepo,
}
