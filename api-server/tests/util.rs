use std::{net::SocketAddr, str::FromStr};

use api_server::{config::Config, run_server};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::{net::TcpListener, task::JoinHandle};

pub const ROUTE_ACCOUNT_REGISTER: &str = "api/account/register";
pub const ROUTE_ACCOUNT_LOG_IN: &str = "api/account/log-in";

pub struct TestServer {
    base_uri: String,
    jh: JoinHandle<anyhow::Result<()>>,
}

impl TestServer {
    pub async fn start() -> Self {
        let cfg = Config {
            bind_addr: SocketAddr::from_str("127.0.0.1:0").unwrap(),
            postgres_dsn: "postgresql://postgres:postgres@127.0.0.1/postgres".to_owned(),
        };

        Self::start_with_config(cfg).await
    }

    pub async fn start_with_config(cfg: Config) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Could not bind `TcpListener`");

        let base_uri = format!(
            "http://{}",
            listener.local_addr().expect("Could not read local address")
        );
        let jh = tokio::spawn(run_server(cfg, listener));

        Self { base_uri, jh }
    }

    pub fn base_uri(&self) -> &str {
        &self.base_uri
    }

    #[allow(unused)]
    pub fn abort(self) {
        self.jh.abort();
    }
}

pub fn random_email() -> String {
    let mut rng = thread_rng();
    let username: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();

    format!("{username}@example.com")
}

pub const USER_ACCOUNT_TEST_DISPLAY_NAME: &str = "Test Account";

pub const USER_ACCOUNT_TEST_PASS_VALID: &str = "valid";
pub const USER_ACCOUNT_TEST_PASS_INVALID: &str = "invalid";
