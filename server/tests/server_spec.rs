use server::startup;
use tokio::net::TcpListener;

#[derive(Debug)]
struct TestApp {
    address: String,
    port: u16,
    name: String,
}

async fn spawn_app() -> Result<TestApp, Box<dyn std::error::Error>> {
    let address = format!("{}:{}", "127.0.0.1", 0);
    let listener = TcpListener::bind(address).await?;
    let local_address = listener.local_addr().unwrap();
    let address = local_address.ip().to_string();
    let port = local_address.port();
    let name = "R2D2";

    println!("Starting app at port {}", &port);

    tokio::task::spawn(async move {
        startup::run(listener, name.to_string())
            .await
            .expect("Failed to start server");
    });

    Ok(TestApp { address, port, name: name.to_string() })
}

#[tokio::test]
async fn server_healthcheck_is_accessible() {
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
async fn server_unknown_url_is_404() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}:{}/random/path", &app.address, &app.port))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn server_root_path_should_show_the_server_name() {
    let app = spawn_app().await.expect("Failed to start the app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "http://{}:{}/",
            &app.address, &app.port
        ))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);

    let content = response.text()
        .await
        .expect("Failed to extract content.");

    assert_eq!(
        format!("Hello from server {}", &app.name),
        content
    )
}