use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use crate::types::CreateNoteReq;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Note {
    pub created_at: String,
    pub last_modified: String,
    pub title: String,
    pub subtitle: String,
    pub texts: String,
    pub footnote: String,
    pub author: String,
    pub category: String,
    pub id: Uuid,
}

impl Note {
    pub fn new(req: CreateNoteReq) -> Self {
        let now = Utc::now();

        let id = Uuid::now_v7();
        Self {
            created_at: now.to_rfc2822(),
            last_modified: now.to_rfc2822(),
            title: req.title,
            subtitle: req.subtitle,
            texts: req.texts,
            footnote: req.footnote,
            author: req.author,
            category: req.category,
            id: id,
        }
    }
}
