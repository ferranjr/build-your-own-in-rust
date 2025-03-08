use crate::domain::models::{Server, Targets};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time;

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
        loop {
            let result = {
                let address = server.read().await.check_status_address();
                reqwest::Client::builder()
                    .timeout(Duration::from_millis(250))
                    .build()
                    .unwrap()
                    .get(address)
                    .send()
                    .await
            };

            let mut server = server.write().await;
            match result {
                Ok(res) => {
                    server.healthy = res.status().is_success();
                }
                Err(_) => {
                    server.healthy = false;
                }
            }
            time::sleep(Duration::from_millis(200)).await;
        }
    }
}
