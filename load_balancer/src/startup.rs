use crate::domain::models::Targets;
use crate::proxy::pipe_through;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

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
            let target = targets.lock()
                .await
                .next_available_server()
                .await;
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
