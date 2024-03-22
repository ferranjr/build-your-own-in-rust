use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use crate::repository::mongo_repository::MongoRepo;
use crate::routes;

pub async fn run_server(tcp_listener: TcpListener) -> std::io::Result<Server> {
    let mongodb = MongoRepo::init().await?;
    let mongodb= web::Data::new(mongodb);

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(mongodb.clone())
            .route("/", web::post().to(routes::create_shortened_url))
            .service(
                web::scope("/private")
                    .route("/status", web::get().to(routes::status))
            )
    })
        .listen(tcp_listener)
        .expect("Failed to start the server")
        .run();

    Ok(http_server)
}