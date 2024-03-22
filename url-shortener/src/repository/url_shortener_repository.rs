use crate::domain::database_models::ShortenedUrl;

pub trait UrlShortenerRepository {
    fn get_shortened_url(&self, key: String) -> impl std::future::Future<Output = std::io::Result<Option<ShortenedUrl>>> + Send;
    fn get_shortened_url_by_full_url(&self, full: &str) -> impl std::future::Future<Output = std::io::Result<Option<ShortenedUrl>>> + Send;
    fn save_shortened_url(&self, shortened_url: ShortenedUrl) -> impl std::future::Future<Output = std::io::Result<ShortenedUrl>> + Send;
}
