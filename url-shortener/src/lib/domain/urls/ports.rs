use std::future::Future;
use url::Url;
use crate::domain::urls::models::short_url::{RepositoryShortUrlError, CreateShortUrlRequest, ShortUrl, ShortUrlId, ShortUrlResponse};

pub trait UrlsService: Clone + Send + Sync + 'static {
    
    fn create_short_url(
        &self,
        req: &CreateShortUrlRequest,
    ) -> impl Future<Output = Result<ShortUrlResponse, RepositoryShortUrlError>> + Send;
    
    fn retrieve_short_url(
        &self,
        key: ShortUrlId
    ) -> impl Future<Output = Result<Option<ShortUrlResponse>, RepositoryShortUrlError>> + Send;
}

pub trait UrlsRepository: Clone + Send + Sync + 'static {
    
    fn create_short_url(
        &self,
        short_url: ShortUrl
    ) -> impl Future<Output = Result<ShortUrl, RepositoryShortUrlError>> + Send;
    
    fn find_short_url_by_long_url(
        &self,
        long_url: Url,
    ) -> impl Future<Output = Result<Option<ShortUrl>, RepositoryShortUrlError>> + Send;

    fn find_short_url_by_short_url(
        &self,
        short_url: Url,
    ) -> impl Future<Output = Result<Option<ShortUrl>, RepositoryShortUrlError>> + Send;

    fn find_short_url_by_key(
        &self,
        key: ShortUrlId,
    ) -> impl Future<Output = Result<Option<ShortUrl>, RepositoryShortUrlError>> + Send;
}