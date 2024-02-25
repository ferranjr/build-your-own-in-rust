use futures_util::future::join;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use server::startup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let address1: SocketAddr = ([127, 0, 0, 1], 8081).into();
    let address2: SocketAddr = ([127, 0, 0, 1], 8082).into();

    let srv1 = async move {
        let listener = TcpListener::bind(address1).await?;
        startup::run(listener, "R2D2".to_string()).await
    };

    let srv2 = async move {
        let listener = TcpListener::bind(address2).await?;
        startup::run(listener, "Chewbacca".to_string()).await
    };

    println!("Listening on http://{} and http://{}", address1, address2);

    let _ = join(srv1, srv2).await;

    Ok(())
}
