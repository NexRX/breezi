use crate::logic::{REGEX_USERNAME, REGEX_UUID};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Sqlite};
use uuid::Uuid;
use validator::Validate;

#[derive(restructed::Models)] // must be separate
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ts_rs::TS, PartialEq, Eq, Validate, JsonSchema)]
#[view(UserRegistration, fields(username, password, email), attributes_with = "all")]
pub struct UserAll {
    #[validate(regex(path = *REGEX_UUID, code = "uuid"))]
    pub id: String,
    #[validate(regex(path = *REGEX_USERNAME, code = "username"))]
    pub username: String,
    #[validate(length(min = 5, max = 1024))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

impl UserRegistration {
    pub async fn insert(&self, conn: impl Executor<'_, Database = Sqlite>) -> crate::Result<String> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO user (id, username, password, email)
            VALUES ($1, $2, $3, $4)",
            id,
            self.username,
            self.password,
            self.email
        )
        .execute(conn)
        .await?;

        Ok(id)
    }
}
