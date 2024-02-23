use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub async fn run(
    tcp_listener: TcpListener,
    name: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!(
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
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(|r| request_handler(r, name.as_str())))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn request_handler(
    req: Request<hyper::body::Incoming>,
    name: &str,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full(format!("Hello from server {}", name)))),
        (&Method::GET, "/private/status") => Ok(Response::new(full("OK"))),
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
