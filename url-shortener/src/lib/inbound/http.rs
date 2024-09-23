pub mod handlers;

use crate::domain::urls::ports::UrlsRepository;
use crate::domain::urls::service::Service;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App};
use std::net::TcpListener;

pub struct HttpServer {
    server: Server,
}

impl HttpServer {
    pub async fn new<R>(urls_service: Service<R>, tcp_listener: TcpListener) -> anyhow::Result<Self>
    where
        R: UrlsRepository,
    {
        let server = run(tcp_listener, urls_service).await?;

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run<R>(
    tcp_listener: TcpListener,
    urls_service: Service<R>,
) -> Result<Server, anyhow::Error>
where
    R: UrlsRepository,
{
    let server = actix_web::HttpServer::new(move || {
        App::new()
            .app_data(Data::new(urls_service.clone()))
            .route(
                "/health_check",
                web::get().to(handlers::monitoring::health_check),
            )
            .route(
                "/",
                web::post().to(handlers::short_urls::create_short_url::<R>),
            )
            .route(
                "/{key}",
                web::get().to(handlers::short_urls::get_short_url::<R>),
            )
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
