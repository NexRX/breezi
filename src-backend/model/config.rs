use crate::Result;
use clap::*;
use clap_config::ClapConfig;
use color_eyre::eyre::bail;
use std::{net::IpAddr, path::PathBuf};

const CONFIG_PATH_DEFAULT: &str = "./config.yaml";

#[derive(ClapConfig, Parser, Debug, Clone)]
pub struct Config {
    /// Binding generation toggle
    #[arg(long, env, default_value_t = true)]
    pub bindings_generate: bool,
    /// Bindings generation directory path
    #[arg(long, env, default_value = "./bindings")]
    pub bindings_dir: PathBuf,

    /// Config file path
    #[arg(short, long, env, default_value = CONFIG_PATH_DEFAULT)]
    pub config_path: PathBuf,

    /// Database (sqlite) file path
    #[arg(short, long, env, default_value = "data/breezi.db")]
    pub database: PathBuf,

    /// Server host binding address.
    #[arg(long, env, default_value = "127.0.0.1")]
    pub server_host: IpAddr,
    /// Server host binding port
    #[arg(short = 'p', long, env, default_value_t = 8080)]
    pub server_port: u16,
    /// Server host CORS (Cross-origin resource sharing) toggle
    #[arg(long, env, default_value_t = true)]
    pub server_cors: bool,
}

impl Config {
    pub fn parse() -> Result<Self> {
        let config_file = match std::fs::read_to_string(CONFIG_PATH_DEFAULT) {
            Ok(config_str) => Some(serde_yaml::from_str(&config_str)?),
            Err(err) if err.raw_os_error().is_some_and(|v| v == 2) => None,
            Err(err) => bail!(err),
        };
        let matches = <Config as CommandFactory>::command().get_matches();
        let config = Config::from_merged(matches, config_file);
        Ok(config)
    }
}
