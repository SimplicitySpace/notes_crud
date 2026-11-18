use crate::note_service::Note;
use crate::types::UpdateNoteReq;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct NoteRepo {
    pool: SqlitePool,
}
impl NoteRepo {
    pub fn new(pool: SqlitePool) -> Self {
        return Self { pool };
    }
    pub async fn save_note(
        &self,
        note: Note,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query(
            "INSERT INTO notes 
            (id, user_id,title, subtitle, texts, footnote, author, category, created_at, last_modified)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&note.id)
        .bind(&note.user_id)

        .bind(&note.title)
        .bind(&note.subtitle)
        .bind(&note.texts)
        .bind(&note.footnote)
        .bind(&note.author)
        .bind(&note.category)
        .bind(&note.created_at)
        .bind(&note.last_modified)
        .execute(&self.pool)
        .await;

        return query;
    }

    pub async fn read_note(&self, note_id: Uuid, user_id: Uuid) -> Result<Option<Note>, sqlx::Error> {
        let query: Result<Option<Note>, sqlx::Error> =
            sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ? AND user_id = ?")
                .bind(note_id)
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await;
        return query;
    }

    pub async fn get_all_notes(&self, user_id: Uuid) -> Result<Vec<Note>, sqlx::Error> {
        let query: Result<Vec<Note>, sqlx::Error> =
            sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE user_id = ? ORDER BY id")
                .bind(user_id)
                .fetch_all(&self.pool)
                .await;
        return query;
    }

    pub async fn update_note(
        &self,
        note_id: Uuid,
        user_id: Uuid,
        update: UpdateNoteReq,
        last_modified: String,
    ) -> Result<Option<Note>, sqlx::Error> {
        let query: Result<Option<Note>, sqlx::Error> = sqlx::query_as::<_, Note>(
            "UPDATE notes SET
                title         = COALESCE(?, title),
                subtitle      = COALESCE(?, subtitle),
                texts         = COALESCE(?, texts),
                footnote      = COALESCE(?, footnote),
                author        = COALESCE(?, author),
                category      = COALESCE(?, category),
                last_modified = ?
             WHERE id = ? AND user_id = ?
             RETURNING *",
        )
        .bind(update.title)
        .bind(update.subtitle)
        .bind(update.texts)
        .bind(update.footnote)
        .bind(update.author)
        .bind(update.category)
        .bind(last_modified)
        .bind(note_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await;
        return query;
    }

    pub async fn delete_note(
        &self,
        note_id: Uuid,
        user_id: Uuid,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> =
            sqlx::query("DELETE FROM notes WHERE id = ? AND user_id = ?")
                .bind(note_id)
                .bind(user_id)
                .execute(&self.pool)
                .await;
        return query;
    }
}
