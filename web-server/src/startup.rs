use crate::domain::http_request::{HttpRequest, Method};
use crate::domain::http_response::{HttpResponse, StatusCodes};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub fn run_server(listener: TcpListener) -> std::io::Result<()> {
    let address = listener.local_addr().unwrap();
    println!(
        "Server started at host {} and port {}",
        address.ip(),
        address.port()
    );
    for stream in listener.incoming() {
        handle_client(stream?).expect("Failed to handle the client request");
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Server received a connection!");
    let buf_reader = BufReader::new(&mut stream);
    let http_request_strings: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let HttpRequest { method, path } = http_request_strings[0].parse::<HttpRequest>()?;

    let http_response = handle_request(method, path)?;
    stream.write_all(http_response.response_string().as_bytes())
}

fn handle_request(method: Method, path: String) -> std::io::Result<HttpResponse> {
    let content = format!("Requested path: {}", path);
    match (method, path) {
        (Method::GET, _) => Ok(HttpResponse::new(StatusCodes::OK, Some(content))),
        (_, _) => Ok(HttpResponse::new(StatusCodes::NotFound, None)),
    }
}
