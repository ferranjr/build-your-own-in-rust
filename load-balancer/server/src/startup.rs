use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::{
    Method,
    Request,
    Response,
    StatusCode,
    body::Bytes,
    server::conn::http1,
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tracing::{error, info, instrument};

pub async fn run(
    tcp_listener: TcpListener,
    name: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(
        "Starting server at: {}:{}",
        &tcp_listener.local_addr()?.ip().to_string(),
        tcp_listener.local_addr()?.port()
    );

    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let io = TokioIo::new(stream);

        let name = name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function into a `Service`
                .serve_connection(io, service_fn(|r| request_handler(r, name.as_str())))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[instrument()]
async fn request_handler(
    req: Request<hyper::body::Incoming>,
    name: &str,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full(format!("Hello from server {}\n", name)))),
        (&Method::GET, "/private/status") => Ok(Response::new(full("OK\n"))),
        (_, _) => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// We create some utility functions to make Empty and Full bodies
// fit our broadened Response body type.
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
