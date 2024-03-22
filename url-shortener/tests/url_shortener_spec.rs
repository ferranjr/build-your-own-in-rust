use std::net::TcpListener;
use nanoid::nanoid;
use url_shortener::domain::http_request::CreateShortUrl;
use url_shortener::startup::run_server;

struct TestApp {
    address: String,
    port: u16
}

async fn spawn_app() -> TestApp {
    let address = format!("{}:{}", "127.0.0.1", 0);
    let tcp_listener = TcpListener::bind(address).unwrap();
    let local_address = tcp_listener.local_addr().unwrap();
    let address = local_address.ip().to_string();
    let port = local_address.port();

    println!("Test App starting at http://{}:{}", address, port);

    tokio::task::spawn(async move {
        run_server(tcp_listener)
            .await
            .expect("Failed to start the server")
            .await
            .expect("Failed to start the server")
    });

    TestApp {
        address,
        port
    }
}

#[tokio::test]
async fn get_private_status_should_return_200 () {
    let test_app = spawn_app().await;
    let response = reqwest::Client::new()
        .get(format!("http://{}:{}/private/status", test_app.address, test_app.port))
        .send()
        .await
        .expect("Failed to connect to healthcheck endpoint");

    assert_eq!(
        response.status().as_u16(),
        200
    );
}

#[tokio::test]
async fn post_create_short_url_should_return_201 () {
    let test_app = spawn_app().await;

    let full_url = format!("https://github.com/ferranjr/build-your-own-in-rust/{}", nanoid!(10));
    let create_short_url = CreateShortUrl {
        url: full_url
    };

    let response = reqwest::Client::new()
        .post(format!("http://{}:{}/", test_app.address, test_app.port))
        .json(&create_short_url)
        .send()
        .await
        .expect("Failed to post a url to be shortened");

    assert_eq!(
        response.status().as_u16(),
        201
    );
}


#[tokio::test]
async fn post_create_short_url_should_be_idempotent_return_201 () {
    let test_app = spawn_app().await;

    let full_url = format!("https://github.com/ferranjr/build-your-own-in-rust/{}", nanoid!(10));
    let create_short_url = CreateShortUrl {
        url: full_url
    };

    let response = reqwest::Client::new()
        .post(format!("http://{}:{}/", test_app.address, test_app.port))
        .json(&create_short_url)
        .send()
        .await
        .expect("Failed to post a url to be shortened");

    assert_eq!(
        response.status().as_u16(),
        201
    );

    let response = reqwest::Client::new()
        .post(format!("http://{}:{}/", test_app.address, test_app.port))
        .json(&create_short_url)
        .send()
        .await
        .expect("Failed to post a url to be shortened");

    assert_eq!(
        response.status().as_u16(),
        201
    );
}