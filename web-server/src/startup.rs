use crate::domain::http_request::{HttpRequest, Method};
use crate::domain::http_response::{HttpResponse, StatusCodes};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

pub fn run_server(listener: TcpListener) -> std::io::Result<()> {
    let address = listener.local_addr().unwrap();
    println!(
        "Server started at host {} and port {}",
        address.ip(),
        address.port()
    );
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream).expect("Failed to handle the client request");
                });
            }
            Err(e) => {
                println!("Something went terribly wrong dealing the incoming stream: {:?}", e)
            }
        }
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
    match (method, path) {
        (Method::GET, path) => match load_file(path) {
            Ok(content) => Ok(HttpResponse::new(StatusCodes::OK, Some(content))),
            Err(_) => Ok(HttpResponse::new(StatusCodes::NotFound, None)),
        },
        (_, _) => Ok(HttpResponse::new(StatusCodes::NotFound, None)),
    }
}

fn load_file(path: String) -> std::io::Result<String> {
    let final_path = std::env::current_dir()?.join(Path::new(&format!("www{}", path)));
    if final_path.is_file() {
        std::fs::read_to_string(final_path)
    } else {
        std::fs::read_to_string(final_path.join("index.html"))
    }
}
