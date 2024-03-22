use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use url_shortener::startup::run_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket_addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0,1)), 8080
    );

    let tcp_listener = TcpListener::bind(socket_addr)?;
    run_server(tcp_listener)
        .await?
        .await?;

    Ok(())
}