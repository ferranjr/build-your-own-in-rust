pub mod handlers;

use std::net::TcpListener;

use actix_web::{App, dev::Server, web, web::Data};
use tracing::info;

use crate::domain::urls::{ports::UrlsRepository, service::Service};

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
    let address = tcp_listener.local_addr()?;
    info!(
        "Starting server at host {} and port {}",
        address.ip(),
        address.port(),
    );

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
            .route(
                "/{key}",
                web::delete().to(handlers::short_urls::delete_short_url::<R>),
            )
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
