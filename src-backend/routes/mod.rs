mod rpc;
mod spa;

use crate::{routes::spa::Spa, Conf};
use axum::routing::Router;
use axum_embed::FallbackBehavior;
use qubit::ServerHandle;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn build(config: &Conf) -> crate::Result<(Router, ServerHandle)> {
    let (rpc_service, rpc_handle) = rpc::router(config).to_service(());

    // Request Router
    let mut axum_router = axum::Router::<()>::new()
        .nest_service("/rpc", rpc_service)
        .route_service(
            // Avoids serving spa when asset not found
            "/assets/{*files}",
            Spa::service(FallbackBehavior::NotFound),
        )
        .layer(TraceLayer::new_for_http())
        .fallback_service(Spa::service(FallbackBehavior::Ok));

    if config.cors_enabled {
        axum_router = axum_router.layer(CorsLayer::permissive());
    }

    Ok((axum_router, rpc_handle))
}
