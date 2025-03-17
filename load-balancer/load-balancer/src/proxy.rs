use http_body_util::{BodyExt, combinators::BoxBody};
use hyper::body::Bytes;
use hyper::client::conn::http1::Builder;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpStream;

pub async fn pipe_through(
    req: Request<hyper::body::Incoming>,
    target: &SocketAddr,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let stream = TcpStream::connect(target).await.unwrap(); // TODO: Deal with this nicely

    let io = TokioIo::new(stream);
    let (mut sender, conn) = Builder::new().handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let resp = sender.send_request(req).await?;
    Ok(resp.map(|b| b.boxed()))
}
