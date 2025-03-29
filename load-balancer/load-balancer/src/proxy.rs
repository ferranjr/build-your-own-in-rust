use std::net::SocketAddr;

use http_body_util::{BodyExt, combinators::BoxBody};
use hyper::{Request, Response, body::Bytes, client::conn::http1::Builder};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tracing::{error, instrument};

#[instrument()]
pub async fn pipe_through(
    req: Request<hyper::body::Incoming>,
    target: &SocketAddr,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let stream = TcpStream::connect(target).await.unwrap(); // TODO: Deal with this nicely

    let io = TokioIo::new(stream);
    let (mut sender, conn) = Builder::new().handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            error!("Connection failed: {:?}", err);
        }
    });

    let resp = sender.send_request(req).await?;
    Ok(resp.map(|b| b.boxed()))
}
