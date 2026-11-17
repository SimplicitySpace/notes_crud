use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use uuid::Uuid;

use crate::{
    types::{Claims, CreateUser, LoginReq, User},
    user_repo::UserRepo,
};

pub struct UserService;

impl UserService {
    pub fn build_user(user: CreateUser) -> Result<User, StatusCode> {
        if user.password != user.confirm_password {
            return Err(StatusCode::BAD_REQUEST);
        }

        let argon2 = Argon2::default();
        let raw_password_in_bytes = user.password.as_bytes();
        let password_hash_attempt = argon2.hash_password(raw_password_in_bytes);
        let hashed_password = match password_hash_attempt {
            Ok(password) => password.to_string(),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
        let id = Uuid::now_v7();

        let u = User {
            first_name: user.first_name,
            last_name: user.last_name,
            user_name: user.user_name,
            hashed_password,
            sex: user.sex,
            nationality: user.nationality,
            age: user.age,
            id,
            token: None,
        };
        Ok(u)
    }

    pub async fn sign_in(state: UserRepo, req: LoginReq) -> Result<User, StatusCode> {
        let user = match UserRepo::grab_user(&state, &req.user_name).await {
            Ok(Some(u)) => u,
            Ok(None) => return Err(StatusCode::UNAUTHORIZED),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let parsed = match PasswordHash::new(&user.hashed_password) {
            Ok(h) => h,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        match Argon2::default().verify_password(req.password.as_bytes(), &parsed) {
            Ok(_) => Ok(user),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    }
    pub fn create_token(user_id: Uuid, secret: &str) -> Result<String, StatusCode> {
        let claims = Claims {
            sub: user_id,
            exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
        };

        let key = match EncodingKey::from_secret(secret.as_bytes()) {
            Ok(k) => k,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
        match encode(&Header::default(), &claims, &key) {
            Ok(token) => Ok(token),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
