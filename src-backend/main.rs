mod config;
mod routes;

use confique::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::{error::Error, net::SocketAddr};
use tokio::net::TcpListener;
use tracing::info;
use crate::config::Conf;

pub type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let config = Conf::builder().env().load()?;

    let (router, qubit_handler) = routes::build(&config)?;

    info!("Starting web server on {}:{}", &config.host, &config.port);
    axum::serve(
        TcpListener::bind(&SocketAddr::from((config.host, config.port)))
            .await
            .unwrap(),
        router,
    )
    .await?;

    qubit_handler.stop()?;
    Ok(())
}
