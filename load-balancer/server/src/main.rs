use server::startup;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server_name = env::var("SERVER_NAME").map_or("R2D2".to_string(), |s| s.to_string());

    let address1: SocketAddr = ([0, 0, 0, 0], 8081).into();
    let listener = TcpListener::bind(address1).await?;
    startup::run(listener, server_name)
        .await
        .expect("Unable to start the server");

    println!("Listening on http://{}", address1);

    Ok(())
}
