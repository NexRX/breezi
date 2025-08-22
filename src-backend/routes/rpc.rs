use qubit::handler;
use qubit::Router;
use tracing::info;

use crate::config::Conf;

#[handler(query)]
async fn hello_world() -> String {
    "Hello, world!".to_string()
}

pub fn router(config: &Conf) -> Router<()> {
    let router = qubit::Router::new().handler(hello_world);

    if config.bindings_generate {
        router.write_bindings_to_dir(&config.bindings_dir);
        info!("Generated Bindings: {:#?}", &config.bindings_dir);
    }

    router
}