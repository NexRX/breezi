mod auth;
mod spa;

use crate::{Config, routes::spa::Spa};
use axum::routing::Router;
use axum_embed::FallbackBehavior;
use qubit::{Extensions, FromRequestExtensions, RpcError, ServerHandle};
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub struct Routes {
    pub axum: Router,
    pub auth: ServerHandle,
}

impl Routes {
    pub fn build(config: Config, pool: SqlitePool) -> crate::Result<Self> {
        let (auth_service, auth) = auth::router(&config).to_service(());

        let is_cors = config.server_cors;

        let mut axum = axum::Router::<()>::new()
            .nest_service("/auth", auth_service)
            .route_service(
                "/assets/{*files}",
                Spa::service(FallbackBehavior::NotFound), // Avoids serving spa when asset not found
            )
            .layer(axum::Extension(Arc::new(config)))
            .layer(axum::Extension(Arc::new(pool)))
            .layer(TraceLayer::new_for_http())
            .fallback_service(Spa::service(FallbackBehavior::Ok));

        if is_cors {
            axum = axum.layer(CorsLayer::permissive());
        }

        Ok(Self { axum, auth })
    }

    pub fn axum(&self) -> Router {
        self.axum.clone()
    }

    pub fn stop_services(self) -> crate::Result {
        self.auth.stop()?;
        Ok(())
    }
}

pub fn fallback_error() -> RpcError {
    RpcError {
        code: qubit::ErrorCode::InternalError,
        message: String::from("Internal Server Error"),
        data: None,
    }
}

/// Axum extension to automatically obtain a database transaction connection on request
///
/// Example:
///
/// ```no_run
/// async fn create_user(Tx(mut tx): Tx) -> String {
///     sqlx::query("INSERT INTO users (name) VALUES (?)")
///         .bind("Alice")
///         .execute(&mut tx)
///         .await
///         .unwrap();
///
///     tx.commit().await.unwrap();
///     "User created".to_string()
/// }
/// ```
#[allow(dead_code)]
#[derive(Debug)]
struct Tx(Transaction<'static, Sqlite>);

impl FromRequestExtensions<()> for Tx {
    async fn from_request_extensions(_: (), extensions: Extensions) -> Result<Self, RpcError> {
        let pool = extensions.get::<SqlitePool>().ok_or_else(fallback_error)?;
        let tx = pool.begin().await.map_err(|_| RpcError {
            code: qubit::ErrorCode::InternalError,
            message: "Internal Server Error - Database Connection".into(),
            data: None,
        })?;

        Ok(Self(tx))
    }
}

/// Axum extension to automatically obtain a auto-commiting database connection on request
///
/// Example:
/// ```no_run
/// async fn list_users(Conn(pool): Conn) -> String {
///     let rows: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM users")
///         .fetch_all(&*pool).await.unwrap();
///
///     format!("Users: {:?}", rows)
/// }
///```
#[derive(Debug)]
struct Conn(SqlitePool);

impl From<&SqlitePool> for Conn {
    fn from(value: &SqlitePool) -> Self {
        Self(value.clone())
    }
}

impl FromRequestExtensions<()> for Conn {
    async fn from_request_extensions(_: (), extensions: Extensions) -> Result<Self, RpcError> {
        let pool = extensions.get::<SqlitePool>().ok_or_else(fallback_error)?.clone();
        Ok(Self(pool))
    }
}
