use confique::Config;
use std::path::PathBuf;
use std::net::IpAddr;

#[derive(Config)]
pub struct Conf {
    /// Bind address.
    #[config(env = "HOST", default = "127.0.0.1")]
    pub host: IpAddr,

    #[config(env = "PORT", default = 8080)]
    pub port: u16,

    #[config(env = "BINDINGS_GENERATE", default = true)]
    pub bindings_generate: bool,

    #[config(env = "BINDINGS_DIR", default = "./bindings")]
    pub bindings_dir: PathBuf,

    #[config(env = "CORS_ENABLED", default = true)]
    pub cors_enabled: bool,
}