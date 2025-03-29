use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;
use web_server::startup::run_server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(addr).await?;

    run_server(listener).await
}
