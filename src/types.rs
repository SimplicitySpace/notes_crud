use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub user_name: String,
    pub password: String,
    pub confirm_password: String,
    pub first_name: String,
    pub last_name: String,
    pub sex: String,
    pub nationality: String,
    pub age: i8,
}
#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_name: String,
    pub hashed_password: String,
    pub first_name: String,
    pub last_name: String,
    pub sex: String,
    pub nationality: String,

    pub age: i8,
    pub id: Uuid,
    pub token: Option<Uuid>,
}
#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // "subject": who the token is for → the user id
    pub exp: usize, // expiry as a unix timestamp; the crate rejects expired tokens automatically
}

#[derive(Serialize)]
pub struct LoginRes {
    pub token: String,
}
