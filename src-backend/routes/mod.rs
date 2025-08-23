mod auth;
mod spa;

use crate::{Config, routes::spa::Spa};
use axum::routing::Router;
use axum_embed::FallbackBehavior;
use qubit::ServerHandle;
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub struct Routes {
    pub axum: Router,
    pub auth: ServerHandle,
}

#[derive(Debug, Clone)]
pub struct Ctx {
    pub config: Config,
    pub pool: SqlitePool,
}

impl Routes {
    pub fn build(config: Config, pool: SqlitePool) -> crate::Result<Self> {
        let is_cors = config.server_cors;
        let ctx = Ctx { config, pool };
        let (auth_service, auth) = auth::router(&ctx.config).to_service(ctx);

        let mut axum = axum::Router::<()>::new()
            .nest_service("/rpc", auth_service)
            .route_service(
                "/assets/{*files}",
                Spa::service(FallbackBehavior::NotFound), // Avoids serving spa when asset not found
            )
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
