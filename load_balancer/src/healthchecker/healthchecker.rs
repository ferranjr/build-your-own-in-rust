use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use crate::domain::models::Server;

pub struct Healthchecker {}

impl Healthchecker {
    pub async fn healthcheck(server: Arc<Mutex<Server>>) {
        loop {

            let result = {
                let address = server.lock().await.check_status_address();
                reqwest::Client::builder()
                    .timeout(Duration::from_millis(200))
                    .build()
                    .unwrap()
                    .get(address)
                    .send()
                    .await
            };

            let mut server = server.lock().await;
            match result {
                Ok(res) => {
                    server.healthy = res.status().is_success();
                },
                Err(_) => {
                    server.healthy = false;
                }
            }
            println!("Server {} health is {}", server.uri, server.healthy);
            time::sleep(Duration::from_millis(100)).await;
        }
    }
}