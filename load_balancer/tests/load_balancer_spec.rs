use load_balancer::startup;
use load_balancer::targets::models::Targets;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Debug)]
struct TestApp {
    address: String,
    port: u16,
    test_server: TestServer,
}

#[derive(Debug)]
struct TestServer {
    address: String,
    port: u16,
    name: String,
}

async fn spawn_server() -> std::io::Result<TestServer> {
    let address = format!("{}:{}", "127.0.0.1", 0);
    let listener = TcpListener::bind(address).await?;
    let local_address = listener.local_addr().unwrap();
    let address = local_address.ip().to_string();
    let port = local_address.port();
    let name = Uuid::new_v4().to_string();

    println!("Starting server {} at port {}", &name, &port);

    let name_2 = name.clone();
    tokio::task::spawn(async move {
        server::startup::run(listener, &name_2)
            .await
            .expect("Failed to start server");
    });

    Ok(TestServer {
        address,
        port,
        name,
    })
}
async fn spawn_app() -> Result<TestApp, Box<dyn std::error::Error>> {
    let address = format!("{}:{}", "127.0.0.1", 0);
    let listener = TcpListener::bind(address).await?;
    let local_address = listener.local_addr().unwrap();
    let address = local_address.ip().to_string();
    let port = local_address.port();

    println!("Starting app at port {}", &port);

    let test_server = spawn_server().await.expect("Failed to start server");
    let server_a_address = format!("{}:{}", &test_server.address, test_server.port);

    let targets = Arc::new(Mutex::new(
        Targets::from_strings(vec![server_a_address]).expect("Failed to init targets"),
    ));

    tokio::task::spawn(async move {
        startup::run(listener, targets)
            .await
            .expect("Failed to start server");
    });

    Ok(TestApp {
        address,
        port,
        test_server,
    })
}

#[tokio::test]
async fn server_healthcheck_is_accessible_through_load_balancer() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "http://{}:{}/private/status",
            &app.address, &app.port
        ))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn server_root_path_through_load_balancer() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}:{}/", &app.address, &app.port))
        .send()
        .await
        .expect("Failed to execute request");

    let status = &response.status().as_u16();

    assert_eq!(*status, 200);

    let content = &response.text().await.expect("Failed to read text");

    assert_eq!(
        *content,
        format!("Hello from server {}", app.test_server.name)
    );
}

#[tokio::test]
async fn server_unknown_url_is_404_through_load_balancer() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}:{}/random/path", &app.address, &app.port))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 404);
}
