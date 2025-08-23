use qubit::Router;
use qubit::handler;
use tracing::info;
use validator::Validate;

use crate::Config;
use crate::model::ErrorResponse;
use crate::model::UserRegistration;
use crate::routes::Conn;

#[handler(mutation)]
async fn register(conn: Conn, user: UserRegistration) -> crate::Result<String, ErrorResponse> {
    user.validate()?;
    Ok(user.insert(&conn.0).await?)
}

pub fn router(config: &Config) -> Router<()> {
    let router = qubit::Router::new().handler(register);

    if config.bindings_generate {
        router.write_bindings_to_dir(&config.bindings_dir);
        info!("Generated Bindings: {:#?}", &config.bindings_dir);
    }

    router
}

#[cfg(test)]
mod test {
    use sqlx::SqlitePool;
    use validator::Validate;

    use crate::model::{ErrorCode, ErrorResponse, UserAll, UserRegistration};

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
            password: "password".into(),
        };

        // When
        let err: ErrorResponse = user_expected.validate().expect_err("user should be invalid").into();

        // Then
        assert_eq!(err.code, ErrorCode::Invalid);
        assert_eq!(err.message, "Invalid field 'email': email")
    }
}
