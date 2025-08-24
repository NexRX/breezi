use qubit::Router;
use qubit::handler;
use tracing::info;
use validator::Validate;

use crate::model::ErrorResponse;
use crate::model::UserRegistration;
use crate::routes::Ctx;

#[handler(mutation)]
async fn register(ctx: Ctx, user: UserRegistration) -> crate::Result<String, ErrorResponse> {
    info!("Registering");
    user.validate()?;
    Ok(user.insert(&ctx.pool).await?)
}

pub fn router() -> Router<Ctx> {
    qubit::Router::<Ctx>::new().handler(register)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde_json::Value;
    use sqlx::SqlitePool;
    use validator::Validate;

    use crate::{
        logic::Invalidation,
        model::{ErrorReason, ErrorResponse, UserAll, UserRegistration},
    };

    #[sqlx::test]
    async fn register_given_valid_then_readable_from_database(pool: SqlitePool) {
        // Given
        let user_expected = UserRegistration {
            username: "username123".into(),
            email: "user@example.com".into(),
            password: "pass".into(),
        };

        // When
        let id = user_expected.insert(&pool).await.expect("user should insert");

        // Then
        let user_actual = sqlx::query_as!(UserAll, r#"SELECT * FROM user WHERE id = $1"#, id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(id, user_actual.id);
        assert_eq!(user_expected, user_actual.into())
    }

    #[tokio::test]
    async fn register_given_invalid_email_then_error_invalid() {
        // Given
        let user_expected = UserRegistration {
            username: "username123".into(),
            email: "user!example.com".into(),
            password: "".into(),
        };

        // When
        let err: ErrorResponse = user_expected.validate().expect_err("user should be invalid").into();

        // Then
        assert_eq!(err.message, "One or more fields are invalid".to_string());
        assert_eq!(
            err.reason,
            ErrorReason::Invalid(HashMap::from([
                (
                    "email".to_string(),
                    Invalidation::new(
                        "email",
                        "Given value is not a valid email",
                        Value::String("user!example.com".into()),
                        []
                    )
                ),
                (
                    "password".to_string(),
                    Invalidation::new(
                        "length",
                        "Given value is not a valid length",
                        Value::String("".into()),
                        [("min".into(), "5".into()), ("max".into(), "1024".into())]
                    )
                ),
            ]))
        );
    }
}
