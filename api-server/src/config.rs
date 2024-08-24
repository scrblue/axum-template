use std::{net::SocketAddr, path::Path};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub postgres_dsn: String,
}

impl Config {
    pub fn read_config(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = std::fs::read_to_string(path)?;
        toml::from_str(&file).map_err(Into::into)
    }
}
