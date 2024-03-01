use load_balancer::domain::models::Targets;
use load_balancer::healthchecks::healthchecker::HealthChecker;
use load_balancer::startup;
use reqwest::Client;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Debug)]
struct TestApp {
    address: String,
    port: u16,
}

impl TestApp {
    async fn get_path(&self, path: String) -> reqwest::Response {
        Client::new()
            .get(format!("http://{}:{}{}", &self.address, &self.port, path))
            .send()
            .await
            .expect("Failed to execute request")
    }
    async fn get_status(&self) -> reqwest::Response {
        self.get_path("/private/status".to_string()).await
    }

    async fn get_root(&self) -> reqwest::Response {
        self.get_path("".to_string()).await
    }
}

#[derive(Debug)]
struct TestServer {
    address: String,
    port: u16,
    name: String,
}

async fn spawn_server(name: String) -> std::io::Result<TestServer> {
    let address = format!("{}:{}", "127.0.0.1", 0);
    let listener = TcpListener::bind(address).await?;
    let local_address = listener.local_addr().unwrap();
    let address = local_address.ip().to_string();
    let port = local_address.port();

    println!("Starting server {} at port {}", &name, &port);

    let name_2 = name.clone();
    tokio::task::spawn(async move {
        server::startup::run(listener, name_2)
            .await
            .expect("Failed to start server");
    });

    Ok(TestServer {
        address,
        port,
        name,
    })
}

async fn spawn_app(test_servers: Vec<SocketAddr>) -> Result<TestApp, Box<dyn std::error::Error>> {
    let address = format!("{}:{}", "127.0.0.1", 0);
    let listener = TcpListener::bind(address).await?;
    let local_address = listener.local_addr().unwrap();
    let address = local_address.ip().to_string();
    let port = local_address.port();

    println!("Starting app at port {}", &port);

    let targets = Arc::new(Mutex::new(Targets::new(test_servers)));

    // Initialises the monitoring of targets
    HealthChecker::init(Arc::clone(&targets)).await;

    // Start the lb
    tokio::task::spawn(async move {
        startup::run(listener, targets)
            .await
            .expect("Failed to start server");
    });

    // Sleep first to ensure healthcheck initialises al servers
    time::sleep(Duration::from_secs(1)).await;

    Ok(TestApp { address, port })
}

#[tokio::test]
async fn server_healthcheck_is_accessible_through_load_balancer() {
    let test_server = spawn_server("R2D2".to_string())
        .await
        .expect("Failed to start server");
    let server_a_address = format!("{}:{}", &test_server.address, test_server.port)
        .parse()
        .expect("Failed to create SocketAddr");

    let app = spawn_app(vec![server_a_address])
        .await
        .expect("Failed to start the app");
    let response = app.get_status().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn server_root_path_through_load_balancer() {
    let test_server = spawn_server("R2D2".to_string())
        .await
        .expect("Failed to start server");
    let server_a_address = format!("{}:{}", &test_server.address, test_server.port)
        .parse()
        .expect("Failed to create SocketAddr");

    let app = spawn_app(vec![server_a_address])
        .await
        .expect("Failed to start the app");

    let response = app.get_root().await;
    let status = &response.status().as_u16();

    assert_eq!(*status, 200);

    let content = &response.text().await.expect("Failed to read text");

    assert_eq!(*content, format!("Hello from server {}", test_server.name));
}

#[tokio::test]
async fn server_unknown_url_is_404_through_load_balancer() {
    let test_server = spawn_server("R2D2".to_string())
        .await
        .expect("Failed to start server");
    let server_a_address = format!("{}:{}", &test_server.address, test_server.port)
        .parse()
        .expect("Failed to create SocketAddr");

    let app = spawn_app(vec![server_a_address])
        .await
        .expect("Failed to start the app");
    let response = app.get_path("/random/path".to_string()).await;

    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn round_robin_works_when_all_servers_healthy() {
    let test_server_a = spawn_server("R2D2".to_string())
        .await
        .expect("Failed to start server");
    let server_a_address = format!("{}:{}", &test_server_a.address, test_server_a.port)
        .parse()
        .expect("Failed to create SocketAddr");

    let test_server_b = spawn_server("Chewbacca".to_string())
        .await
        .expect("Failed to start server");
    let server_b_address = format!("{}:{}", &test_server_b.address, test_server_b.port)
        .parse()
        .expect("Failed to create SocketAddr");

    let app = spawn_app(vec![server_a_address, server_b_address])
        .await
        .expect("Failed to start the app");

    let content_1 = app
        .get_root()
        .await
        .text()
        .await
        .expect("Failed to read text");

    assert_eq!(
        content_1,
        format!("Hello from server {}", test_server_a.name)
    );

    let content_2 = app
        .get_root()
        .await
        .text()
        .await
        .expect("Failed to read text");

    assert_eq!(
        content_2,
        format!("Hello from server {}", test_server_b.name)
    );

    let content_3 = app
        .get_root()
        .await
        .text()
        .await
        .expect("Failed to read text");

    assert_eq!(
        content_3,
        format!("Hello from server {}", test_server_a.name)
    );
}

#[tokio::test]
async fn round_robin_skips_unhealthy_server() {
    let test_server_a = spawn_server("R2D2".to_string())
        .await
        .expect("Failed to start server");
    let server_a_address = format!("{}:{}", &test_server_a.address, test_server_a.port)
        .parse()
        .expect("Failed to create SocketAddr");

    let server_not_running = {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind address");
        listener.local_addr().unwrap()
    };

    let app = spawn_app(vec![server_a_address, server_not_running])
        .await
        .expect("Failed to start the app");

    let content_1 = app
        .get_root()
        .await
        .text()
        .await
        .expect("Failed to read text");

    assert_eq!(
        content_1,
        format!("Hello from server {}", test_server_a.name)
    );

    let content_2 = app
        .get_root()
        .await
        .text()
        .await
        .expect("Failed to read text");

    assert_eq!(
        content_2,
        format!("Hello from server {}", test_server_a.name)
    );

    let content_3 = app
        .get_root()
        .await
        .text()
        .await
        .expect("Failed to read text");

    assert_eq!(
        content_3,
        format!("Hello from server {}", test_server_a.name)
    );
}
