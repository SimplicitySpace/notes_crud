use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{state::AppState, types::Claims};

pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = match parts.headers.get("AUTHORIZATION") {
            Some(v) => v,
            None => return Err(StatusCode::UNAUTHORIZED),
        };

        let header_str = match header.to_str() {
            Ok(s) => s,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        };
        let token = match header_str.strip_prefix("Bearer ") {
            Some(t) => t,
            None => return Err(StatusCode::UNAUTHORIZED),
        };

        let key = match DecodingKey::from_secret(state.jwt_secret.as_bytes()) {
            Ok(k) => k,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        };

        match decode::<Claims>(token, &key, &Validation::default()) {
            Ok(data) => Ok(AuthUser {
                user_id: data.claims.sub,
            }),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
