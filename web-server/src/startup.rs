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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let first_line = http_request[0].to_string();
    let path = first_line.split(' ').collect::<Vec<&str>>()[1];

    let content = format!("Requested path: {}", path);
    let content_length_line = format!("Content-Length: {}", content.len());
    let status_code_line = "HTTP/1.1 200 OK";

    stream.write_all(
        format!(
            "{}\r\n{}\r\n\r\n{}\r\n",
            status_code_line, content_length_line, content
        )
        .as_bytes(),
    )
}
