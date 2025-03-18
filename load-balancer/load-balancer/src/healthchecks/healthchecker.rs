use crate::domain::models::{Server, Targets};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time;
use tracing::error;

pub struct HealthChecker {}

impl HealthChecker {
    pub async fn init(targets: Arc<Mutex<Targets>>) {
        let servers = &targets.lock().await.servers;
        for server in servers.iter() {
            let server = Arc::clone(server);
            tokio::spawn(async { HealthChecker::healthcheck(server).await });
        }
    }

    pub async fn healthcheck(server: Arc<RwLock<Server>>) {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(50))
            .build()
            .unwrap();

        let base_url = server.read().await.uri.to_string();
        let address = server.read().await.check_status_address();

        loop {
            let result = client.get(address.clone()).send().await;

            let mut server = server.write().await;
            server.healthy = match result {
                Ok(res) => res.status().is_success(),
                Err(_) => {
                    error!("Unable to check status {}", base_url);
                    false
                }
            };

            time::sleep(Duration::from_millis(50)).await;
        }
    }
}
