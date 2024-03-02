use crate::domain::http_request::{HttpRequest, Method};
use crate::domain::http_response::{HttpResponse, StatusCodes};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub async fn run_server(listener: TcpListener) -> std::io::Result<()> {
    let address = listener.local_addr().unwrap();
    println!(
        "Server started at host {} and port {}",
        address.ip(),
        address.port()
    );
    loop {
        let (stream, _) = listener.accept().await?;
        handle_client(stream).await?;
    }
}

async fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    println!("Server received a connection!");
    let mut stream = BufReader::new(stream);

    let mut first_line = String::new();
    stream.read_line(&mut first_line).await.unwrap();

    let HttpRequest { method, path } = first_line.parse::<HttpRequest>()?;
    let http_response = handle_request(method, path).await?;
    stream
        .write_all(http_response.response_string().as_bytes())
        .await
}

async fn handle_request(method: Method, path: String) -> std::io::Result<HttpResponse> {
    match (method, path) {
        (Method::GET, path) => match load_file(path).await {
            Ok(content) => Ok(HttpResponse::new(StatusCodes::OK, Some(content))),
            Err(_) => Ok(HttpResponse::new(StatusCodes::NotFound, None)),
        },
        (_, _) => Ok(HttpResponse::new(StatusCodes::NotFound, None)),
    }
}

async fn load_file(path: String) -> std::io::Result<String> {
    let final_path = std::env::current_dir()?.join(Path::new(&format!("www{}", path)));
    if final_path.is_file() {
        tokio::fs::read_to_string(final_path).await
    } else {
        tokio::fs::read_to_string(final_path.join("index.html")).await
    }
}
