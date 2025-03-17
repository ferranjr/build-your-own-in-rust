use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::{Client, IndexModel};
use nanoid::nanoid;
use serde_json::json;
use std::net::TcpListener;
use testcontainers::core::IntoContainerPort;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerRequest, ImageExt};
use testcontainers_modules::mongo::Mongo;
use url_shortener::domain::urls::models::short_url::ShortUrl;
use url_shortener::domain::urls::service::ServiceConfig;
use url_shortener::inbound::http::HttpServer;
use url_shortener::inbound::http::handlers::short_urls::CreateShortUrlResponse;
use url_shortener::outbound::mongo::{MongoClient, MongoDatabase};

#[derive(Debug)]
struct TestApp {
    base_url: String,
}

#[derive(Debug)]
struct TestMongo {
    mongo_uri: String,
    mongo_db_name: String,
}

async fn set_up_database(mongo_uri: &str) -> Result<TestMongo, Box<dyn std::error::Error>> {
    // Set up indices and DB
    let client: Client = Client::with_uri_str(mongo_uri).await.unwrap();
    let mongo_db_name: String = format!("short_urls_db_{}", nanoid!(6));
    let col_name: &str = "short_urls";
    let database = client.database(mongo_db_name.as_str());
    let _ = database.create_collection(col_name).await.unwrap();
    let collection = database.collection::<ShortUrl>(col_name);

    let index_long_url = IndexModel::builder()
        .keys(doc! {"long_url": 1})
        .options(IndexOptions::builder().unique(true).build())
        .build();
    let index_key = IndexModel::builder()
        .keys(doc! {"key": 1})
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection
        .create_indexes(vec![index_long_url, index_key])
        .await
        .unwrap();

    Ok(TestMongo {
        mongo_uri: mongo_uri.to_string(),
        mongo_db_name,
    })
}

async fn spawn_app(test_mongo: TestMongo) -> Result<TestApp, Box<dyn std::error::Error>> {
    let address = format!("{}:{}", "127.0.0.1", 0);
    let listener = TcpListener::bind(address)?;
    let local_address = listener.local_addr().unwrap();
    let address = local_address.ip().to_string();
    let port = local_address.port();

    let TestMongo {
        mongo_uri,
        mongo_db_name,
    } = test_mongo;
    let mongo = MongoClient::new(mongo_uri.as_str()).await?;
    let mongo_repository = MongoDatabase::new(mongo, mongo_db_name.as_str());

    let base_url = format!("http://{address}:{port}/");

    let service_config = ServiceConfig::new(3, base_url.as_str());
    let urls_service =
        url_shortener::domain::urls::service::Service::new(mongo_repository, service_config);

    // Create HttpServer
    let http_server = HttpServer::new(urls_service, listener).await?;

    let _ = tokio::spawn(http_server.run_until_stopped());

    Ok(TestApp { base_url })
}

fn container_request() -> ContainerRequest<Mongo> {
    Mongo::default().with_name("mongo").with_tag("8.0.0")
}

#[tokio::test]
async fn healthcheck_should_be_ok() {
    let mongodb = container_request().start().await.unwrap();
    let host_ip = mongodb.get_host().await.unwrap();
    let host_port = mongodb.get_host_port_ipv4(27017.tcp()).await.unwrap();
    let mongo_uri = format!("mongodb://{host_ip}:{host_port}");

    // Set up indices and DB
    let test_mongo = set_up_database(mongo_uri.as_str()).await.unwrap();

    let test_app = spawn_app(test_mongo).await.unwrap();
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}health_check", &test_app.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn fail_with_400_for_non_valid_urls() {
    let mongodb = container_request().start().await.unwrap();
    let host_ip = mongodb.get_host().await.unwrap();
    let host_port = mongodb.get_host_port_ipv4(27017.tcp()).await.unwrap();
    let mongo_uri = format!("mongodb://{host_ip}:{host_port}");

    // Set up indices and DB
    let test_mongo = set_up_database(mongo_uri.as_str()).await.unwrap();

    let test_app = spawn_app(test_mongo).await.unwrap();
    let client = reqwest::Client::new();

    //Act
    let response = client
        .post(&test_app.base_url)
        .json(&json!({
            "long_url": "invalid_url"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 400);
    assert!(
        response
            .text()
            .await
            .unwrap()
            .contains("Json deserialize error")
    );
}

#[tokio::test]
async fn success_for_a_valid_url() {
    let mongodb = container_request().start().await.unwrap();
    let host_ip = mongodb.get_host().await.unwrap();
    let host_port = mongodb.get_host_port_ipv4(27017.tcp()).await.unwrap();
    let mongo_uri = format!("mongodb://{host_ip}:{host_port}");

    // Set up indices and DB
    let test_mongo = set_up_database(mongo_uri.as_str()).await.unwrap();

    let test_app = spawn_app(test_mongo).await.unwrap();
    let client = reqwest::Client::new();

    //Act
    let response = client
        .post(&test_app.base_url)
        .json(&json!({
            "long_url": "https://www.mongodb.com/docs/drivers/rust/current/fundamentals/indexes/"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn idempotency() {
    let mongodb = container_request().start().await.unwrap();
    let host_ip = mongodb.get_host().await.unwrap();
    let host_port = mongodb.get_host_port_ipv4(27017.tcp()).await.unwrap();
    let mongo_uri = format!("mongodb://{host_ip}:{host_port}");

    // Set up indices and DB
    let test_mongo = set_up_database(mongo_uri.as_str()).await.unwrap();

    let test_app = spawn_app(test_mongo).await.unwrap();
    let client = reqwest::Client::new();

    //Act
    let response_one = client
        .post(&test_app.base_url)
        .json(&json!({
            "long_url": "https://www.mongodb.com/docs/drivers/rust/current/fundamentals/indexes/"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    let response_two = client
        .post(&test_app.base_url)
        .json(&json!({
            "long_url": "https://www.mongodb.com/docs/drivers/rust/current/fundamentals/indexes/"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response_one.status().as_u16(), 201);
    assert_eq!(response_two.status().as_u16(), 201);

    let create_response_one: CreateShortUrlResponse = response_one.json().await.unwrap();
    let create_response_two: CreateShortUrlResponse = response_two.json().await.unwrap();
    assert_eq!(create_response_one, create_response_two);
}

#[tokio::test]
async fn success_for_redirection_to_long_url_that_exists() {
    let mongodb = container_request().start().await.unwrap();
    let host_ip = mongodb.get_host().await.unwrap();
    let host_port = mongodb.get_host_port_ipv4(27017.tcp()).await.unwrap();
    let mongo_uri = format!("mongodb://{host_ip}:{host_port}");

    // Set up indices and DB
    let test_mongo = set_up_database(mongo_uri.as_str()).await.unwrap();

    let test_app = spawn_app(test_mongo).await.unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    //Act
    let response_one = client
        .post(&test_app.base_url)
        .json(&json!({
            "long_url": "https://www.mongodb.com/docs/drivers/rust/current/fundamentals/indexes/"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response_one.status().as_u16(), 201);
    let create_response_one: CreateShortUrlResponse = response_one.json().await.unwrap();

    let response_two = client
        .get(create_response_one.short_url().to_string())
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response_two.status().as_u16(), 303);
    assert_eq!(
        response_two.headers().get("Location").unwrap(),
        create_response_one.long_url().as_str()
    );
}

#[tokio::test]
async fn fail_404_for_redirection_to_long_url_that_doesnt_exists() {
    let mongodb = container_request().start().await.unwrap();
    let host_ip = mongodb.get_host().await.unwrap();
    let host_port = mongodb.get_host_port_ipv4(27017.tcp()).await.unwrap();
    let mongo_uri = format!("mongodb://{host_ip}:{host_port}");

    // Set up indices and DB
    let test_mongo = set_up_database(mongo_uri.as_str()).await.unwrap();

    let test_app = spawn_app(test_mongo).await.unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    //Act
    let response = client
        .get(format!("{}{}", &test_app.base_url, "FOOBAR"))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn successfully_delete_url() {
    let mongodb = container_request().start().await.unwrap();
    let host_ip = mongodb.get_host().await.unwrap();
    let host_port = mongodb.get_host_port_ipv4(27017.tcp()).await.unwrap();
    let mongo_uri = format!("mongodb://{host_ip}:{host_port}");

    // Set up indices and DB
    let test_mongo = set_up_database(mongo_uri.as_str()).await.unwrap();

    let test_app = spawn_app(test_mongo).await.unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let response_one = client
        .post(&test_app.base_url)
        .json(&json!({
            "long_url": "https://www.mongodb.com/docs/drivers/rust/current/fundamentals/indexes/"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response_one.status().as_u16(), 201);
    let create_response_one: CreateShortUrlResponse = response_one.json().await.unwrap();

    let response_find_one = client
        .get(create_response_one.short_url().to_owned())
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response_find_one.status().as_u16(), 303);

    let response_deletion = client
        .delete(create_response_one.short_url().as_str())
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response_deletion.status().as_u16(), 200);

    let response_find_two = client
        .get(create_response_one.short_url().to_owned())
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response_find_two.status().as_u16(), 404);
}

#[tokio::test]
async fn fail_delete_non_existing_url() {
    let mongodb = container_request().start().await.unwrap();
    let host_ip = mongodb.get_host().await.unwrap();
    let host_port = mongodb.get_host_port_ipv4(27017.tcp()).await.unwrap();
    let mongo_uri = format!("mongodb://{host_ip}:{host_port}");

    // Set up indices and DB
    let test_mongo = set_up_database(mongo_uri.as_str()).await.unwrap();

    let test_app = spawn_app(test_mongo).await.unwrap();
    let client = reqwest::Client::new();

    let response_deletion = client
        .delete(format!("{}{}", &test_app.base_url, "FOOBAR"))
        .json("")
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response_deletion.status().as_u16(), 404)
}
