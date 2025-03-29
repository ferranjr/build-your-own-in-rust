use std::sync::Arc;

use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::{error, info};

use crate::{domain::models::Targets, proxy::pipe_through};

pub async fn run(
    tcp_listener: TcpListener,
    targets: Arc<Mutex<Targets>>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Starting load balancer at: {}:{}",
        &tcp_listener.local_addr()?.ip().to_string(),
        tcp_listener.local_addr()?.port()
    );
    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let io = TokioIo::new(stream);

        let targets = Arc::clone(&targets);
        tokio::task::spawn(async move {
            let target = targets.lock().await.next_available_server().await;
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(|r| pipe_through(r, &target)))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
