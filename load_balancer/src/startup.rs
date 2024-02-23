use crate::targets::models::Targets;
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::body::Bytes;
use hyper::client::conn::http1::Builder;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

pub async fn run(
    tcp_listener: TcpListener,
    targets: Arc<Mutex<Targets>>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Starting load balancer at: {}:{}",
        &tcp_listener.local_addr()?.ip().to_string(),
        tcp_listener.local_addr()?.port()
    );

    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let io = TokioIo::new(stream);

        let targets = Arc::clone(&targets);
        tokio::task::spawn(async move {
            let target = targets.lock().unwrap().next();
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(|r| pipe_through(r, &target)))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn pipe_through(
    req: Request<hyper::body::Incoming>,
    target: &SocketAddr,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let stream = TcpStream::connect(target).await.unwrap(); // TODO: Deal with this nicely

    let io = TokioIo::new(stream);
    let (mut sender, conn) = Builder::new()
        // .preserve_header_case(true)
        // .title_case_headers(true)
        .handshake(io)
        .await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let resp = sender.send_request(req).await?;
    Ok(resp.map(|b| b.boxed()))
}
