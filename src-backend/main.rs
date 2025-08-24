mod logic;
mod model;
mod routes;

use crate::{logic::setup_database, model::Config, routes::Routes};
use color_eyre::eyre::Report;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type Result<T = (), E = Report> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).init();
    color_eyre::install()?;

    info!("Starting up...");
    let config = Config::parse()?;
    crate::logic::generate_all_bindings(&config)?;
    let pool = setup_database(&config).await?;
    let router = Routes::build(config.clone(), pool)?;

    info!("Starting web server on {}:{}", &config.server_host, &config.server_port);
    axum::serve(
        TcpListener::bind(&SocketAddr::from((config.server_host, config.server_port))).await?,
        router.axum(),
    )
    .await?;

    info!("Stopping...");
    router.stop_services()?;
    Ok(())
}
