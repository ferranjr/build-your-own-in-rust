use std::{env, net::SocketAddr};

use server::startup;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let server_name = env::var("SERVER_NAME").map_or("R2D2".to_string(), |s| s.to_string());

    let address1: SocketAddr = ([0, 0, 0, 0], 8081).into();
    let listener = TcpListener::bind(address1).await?;
    startup::run(listener, server_name)
        .await
        .expect("Unable to start the server");

    info!("Listening on http://{}", address1);

    Ok(())
}
