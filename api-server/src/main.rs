use std::path::PathBuf;

use clap::Parser;
use tokio::net::TcpListener;

use api_server::{config::Config, run_server};

#[derive(Parser)]
struct Args {
    config_path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().pretty().init();

    let Args { config_path } = Args::parse();
    let config = Config::read_config(&config_path)?;

    let listener = TcpListener::bind(config.bind_addr).await?;
    run_server(config, listener).await?;

    Ok(())
}
