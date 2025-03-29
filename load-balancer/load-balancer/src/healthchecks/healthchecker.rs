use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{Mutex, RwLock},
    time,
};
use tracing::{error, info};

use crate::domain::models::{Server, Targets};

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
                Ok(res) => {
                    let result = res.status().is_success();
                    if !server.healthy {
                        info!("Server is back up: {}", base_url);
                    }
                    result
                }
                Err(_) => {
                    if server.healthy {
                        error!("Server gone down, unable to check status: {}", base_url);
                    }
                    false
                }
            };

            time::sleep(Duration::from_millis(50)).await;
        }
    }
}
