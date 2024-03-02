use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use web_server::startup::run_server;

fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(addr).unwrap();

    run_server(listener).unwrap()
}
