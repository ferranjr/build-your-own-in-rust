use domain::models::Targets;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

mod domain;
mod proxy;
mod startup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("{}:{}", "127.0.0.1", 8080);
    let listener = TcpListener::bind(address).await?;
    let targets: Arc<Mutex<Targets>> = Arc::new(Mutex::new(
        Targets::from_strings(vec!["127.0.0.1:8081".into(), "127.0.0.1:8082".into()])
            .expect("Failed to init domain"),
    ));

    startup::run(listener, targets).await
}
