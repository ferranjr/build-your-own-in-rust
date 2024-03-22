use std::fmt::{Display, Formatter};
use actix_web::{HttpResponse, ResponseError, web};
use actix_web::http::StatusCode;
use nanoid::nanoid;
use crate::domain::database_models::ShortenedUrl;
use crate::domain::http_request::CreateShortUrl;
use crate::repository::mongo_repository::MongoRepo;
use crate::repository::url_shortener_repository::UrlShortenerRepository;

#[derive(Debug)]
pub enum CreateUrlError {
    UnexpectedError,
}

impl Display for CreateUrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for CreateUrlError {
    fn status_code(&self) -> StatusCode {
        match self {
            CreateUrlError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn create_shortened_url(
    mongodb: web::Data<MongoRepo>,
    create_short_url: web::Json<CreateShortUrl>,
) -> Result<HttpResponse, CreateUrlError>  {
    let mongodb = mongodb.into_inner();
    let full_url = create_short_url.url.as_str();
    if let Some(_shorten_url) = mongodb.get_shortened_url_by_full_url(full_url)
        .await
        .unwrap() {
       return Ok(HttpResponse::new(StatusCode::CREATED));
    }

    // Create new entry to save
    let key = nanoid!(8);
    mongodb.save_shortened_url(
        ShortenedUrl {
            key: key.clone(),
            full: full_url.to_string(),
            short: format!("http://localhost:8080/{}", &key),
        }
    ).await.map_err(|_| CreateUrlError::UnexpectedError )?;

    Ok(HttpResponse::new(StatusCode::CREATED))
}