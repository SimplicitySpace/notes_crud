use sqlx::SqlitePool;

use crate::types::{ User};
#[derive(Clone)]
pub struct UserRepo {
    pub pool: SqlitePool,
}

impl UserRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool: pool }
    }

    pub async fn save_new_user(
        &self,
        user: User,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query(
            "INSERT INTO users
            (id, user_name, hashed_password, first_name, last_name, sex, nationality, age, token)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&user.id)
        .bind(&user.user_name)
        .bind(&user.hashed_password)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.sex)
        .bind(&user.nationality)
        .bind(&user.age)
        .bind(&user.token)
        .execute(&self.pool)
        .await;

        return query;
    }

    pub async fn grab_user(&self, user_name: &str) -> Result<Option<User>, sqlx::Error> {
        let query: Result<Option<User>, sqlx::Error> =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_name = ?")
                .bind(user_name)
                .fetch_optional(&self.pool)
                .await;

        return query;
    }
}
