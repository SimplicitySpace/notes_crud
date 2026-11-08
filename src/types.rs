use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Note {
    pub created_at: String,
    pub last_modified: String,
    pub title: String,
    pub subtitle: String,
    pub texts: String,
    pub footnote: String,
    pub author: String,
    pub category: String,
    pub id: usize,
}
#[derive(Debug, Deserialize)]
pub struct CreateNoteReq {
    pub title: String,
    pub subtitle: String,
    pub texts: String,
    pub footnote: String,
    pub author: String,
    pub category: String,
}
#[derive(Debug, Deserialize)]
pub struct UpdateNoteReq {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub texts: Option<String>,
    pub footnote: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
}
