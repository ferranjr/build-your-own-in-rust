use std::net::TcpListener;
use std::thread;
use web_server::startup::run_server;

struct TestApp {
    address: String,
    port: u16,
}

async fn spawn_app() -> std::io::Result<TestApp> {
    let address = format!("{}:{}", "127.0.0.1", 0);
    let listener = TcpListener::bind(address)?;
    let local_address = listener.local_addr().unwrap();
    let address = local_address.ip().to_string();
    let port = local_address.port();

    thread::spawn(move || {
        println!("Spinning up the test server at port {}", port);
        run_server(listener).expect("Unable to start server");
    });

    Ok(TestApp { address, port })
}

#[tokio::test]
async fn server_should_listen_for_connections() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}:{}/", &app.address, &app.port))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn server_should_respond_with_path() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}:{}/aloha", &app.address, &app.port))
        .send()
        .await
        .expect("Failed to execute request");

    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let content = response
        .text()
        .await
        .expect("Failed to extract response content.");
    assert_eq!("Requested path: /aloha".to_string(), content);
}

#[tokio::test]
async fn server_should_respond_with_root_path() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}:{}/", &app.address, &app.port))
        .send()
        .await
        .expect("Failed to execute request");

    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let content = response
        .text()
        .await
        .expect("Failed to extract response content.");
    assert_eq!("Requested path: /".to_string(), content);
}

#[tokio::test]
async fn server_should_respond_not_found_for_invalid_path() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "http://{}:{}/invalid_path",
            &app.address, &app.port
        ))
        .send()
        .await
        .expect("Failed to execute request");

    let status = response.status().as_u16();
    assert_eq!(status, 404);
}
