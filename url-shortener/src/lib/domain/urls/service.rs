use crate::domain::urls::models::short_url::{
    CreateShortUrlRequest, RepositoryShortUrlError, ShortUrl, ShortUrlId, ShortUrlResponse,
};
use crate::domain::urls::ports::{UrlsRepository, UrlsService};
use url::Url;

#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: UrlsRepository,
{
    repo: R,
    config: ServiceConfig,
}

impl<R> Service<R>
where
    R: UrlsRepository,
{
    pub fn new(repo: R, config: ServiceConfig) -> Self {
        Self { repo, config }
    }
}

impl<R> UrlsService for Service<R>
where
    R: UrlsRepository,
{
    async fn create_short_url(
        &self,
        req: &CreateShortUrlRequest,
    ) -> Result<ShortUrlResponse, RepositoryShortUrlError> {
        // First we check if already in our repository
        let maybe_short_url = self
            .repo
            .find_short_url_by_long_url(req.long_url().to_owned())
            .await?;

        if let Some(short_url) = maybe_short_url {
            let shorten_url: Url =
                Url::parse(format!("{}{}", self.config.base_url, short_url.key()).as_str())?;
            return Ok(ShortUrlResponse::new(
                short_url.key().to_owned(),
                short_url.long_url().to_owned(),
                shorten_url,
            ));
        }

        let mut retries = self.config.retries().to_owned();
        loop {
            let short_url = ShortUrl::from_long_url(req.long_url())?;
            match self.repo.create_short_url(short_url).await {
                Ok(value) => {
                    return Ok(ShortUrlResponse::from(
                        value,
                        self.config.base_url.to_owned(),
                    )?);
                }
                Err(RepositoryShortUrlError::Duplicate { .. }) if retries > 0 => retries -= 1,
                Err(error) => return Err(error),
            }
        }
    }

    async fn retrieve_short_url(
        &self,
        key: ShortUrlId,
    ) -> Result<Option<ShortUrlResponse>, RepositoryShortUrlError> {
        let result = self.repo.find_short_url_by_key(key).await?;
        let base_url = &self.config.base_url;

        match result {
            None => Ok(None),
            Some(sh) => Ok(Some(ShortUrlResponse::from(sh, base_url.to_owned())?)),
        }
    }

    async fn delete_short_url(&self, key: ShortUrlId) -> Result<(), RepositoryShortUrlError> {
        self.repo.delete_short_url_by_key(key).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    retries: u8,
    base_url: String,
}

impl ServiceConfig {
    pub fn new(retries: u8, base_url: &str) -> Self {
        Self {
            retries,
            base_url: base_url.to_string(),
        }
    }

    pub fn retries(&self) -> &u8 {
        &self.retries
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::urls::models::short_url::{
        CreateShortUrlRequest, RepositoryShortUrlError, ShortUrl, ShortUrlId, ShortUrlResponse,
    };
    use crate::domain::urls::ports::{UrlsRepository, UrlsService};
    use crate::domain::urls::service::{Service, ServiceConfig};
    use mockall::mock;
    use url::Url;

    mock! {
        pub MockUrlsRepository { }

        impl UrlsRepository for MockUrlsRepository {
            async fn create_short_url(&self, short_url: ShortUrl) -> Result<ShortUrl, RepositoryShortUrlError>;
            async fn find_short_url_by_long_url(&self, long_url: Url) -> Result<Option<ShortUrl>, RepositoryShortUrlError>;
            async fn find_short_url_by_short_url(&self, short_url: Url) -> Result<Option<ShortUrl>, RepositoryShortUrlError>;
            async fn find_short_url_by_key(&self, key: ShortUrlId) -> Result<Option<ShortUrl>, RepositoryShortUrlError>;
            async fn delete_short_url_by_key(&self, key: ShortUrlId) -> Result<(), RepositoryShortUrlError>;
        }

        impl Clone for MockUrlsRepository {
            fn clone(&self) -> Self;
        }
    }

    fn service_config() -> ServiceConfig {
        ServiceConfig::new(3, "https://tinyurl.uk/")
    }

    #[tokio::test]
    async fn repository_failure_is_passed_to_caller() {
        let mut mock_urls_repository = MockMockUrlsRepository::new();

        mock_urls_repository
            .expect_find_short_url_by_long_url()
            .times(1)
            .returning(|_| Ok(None));

        mock_urls_repository
            .expect_create_short_url()
            .times(1)
            .returning(|_| Err(RepositoryShortUrlError::ShortUrlNotFound));

        let service_under_test = Service::new(mock_urls_repository, service_config());
        let test_url =
            Url::parse("https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust")
                .unwrap();
        let create_short_url_request = CreateShortUrlRequest::new(test_url);
        let result = service_under_test
            .create_short_url(&create_short_url_request)
            .await;

        let short_url_error = result.unwrap_err();
        assert_eq!(
            short_url_error.to_string(),
            RepositoryShortUrlError::ShortUrlNotFound.to_string()
        )
    }

    #[tokio::test]
    async fn create_entry_successfully_and_return_short_url_to_caller() {
        let test_url =
            Url::parse("https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust")
                .unwrap();
        let short_url = ShortUrl::from_long_url(&test_url).unwrap();
        let mut mock_urls_repository = MockMockUrlsRepository::new();

        mock_urls_repository
            .expect_find_short_url_by_long_url()
            .times(1)
            .returning(|_| Ok(None));

        mock_urls_repository
            .expect_create_short_url()
            .times(1)
            .returning(move |_| Ok(short_url.to_owned()));

        let service_under_test = Service::new(mock_urls_repository, service_config());

        let create_short_url_request = CreateShortUrlRequest::new(test_url.to_owned());
        let result = service_under_test
            .create_short_url(&create_short_url_request)
            .await
            .unwrap();
        let key = result.key();

        let result_short_url = result.short_url();
        let result_long_url = result.long_url();

        let expected_short_url =
            Url::parse(format!("{}{}", service_config().base_url, key).as_str()).unwrap();

        assert_eq!(result_long_url.as_str(), test_url.as_str());
        assert_eq!(result_short_url.as_str(), expected_short_url.as_str())
    }

    #[tokio::test]
    async fn create_entry_for_existing_long_url_must_return_same_short_url_to_caller() {
        let test_url =
            Url::parse("https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust")
                .unwrap();
        let short_url = ShortUrl::from_long_url(&test_url).unwrap();

        let mut mock_urls_repository = MockMockUrlsRepository::new();

        mock_urls_repository
            .expect_find_short_url_by_long_url()
            .times(1)
            .returning(move |_| Ok(Some(short_url.to_owned())));

        let service_under_test = Service::new(mock_urls_repository, service_config());

        let create_short_url_request = CreateShortUrlRequest::new(test_url.to_owned());
        let result = service_under_test
            .create_short_url(&create_short_url_request)
            .await
            .unwrap();
        let key = result.key();

        let result_short_url = result.short_url();
        let result_long_url = result.long_url();

        let expected_short_url =
            Url::parse(format!("{}{}", service_config().base_url, key).as_str()).unwrap();

        assert_eq!(result_long_url.as_str(), test_url.as_str());
        assert_eq!(result_short_url.as_str(), expected_short_url.as_str())
    }

    #[tokio::test]
    async fn create_instance_work_even_if_duplicate_error_happens_less_than_max_retries() {
        let test_url =
            Url::parse("https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust")
                .unwrap();
        let short_url = ShortUrl::from_long_url(&test_url).unwrap();

        let mut mock_urls_repository = MockMockUrlsRepository::new();

        mock_urls_repository
            .expect_find_short_url_by_long_url()
            .times(1)
            .returning(|_| Ok(None));

        mock_urls_repository
            .expect_create_short_url()
            .times(2)
            .returning(|_| {
                Err(RepositoryShortUrlError::Duplicate {
                    short_url_id: ShortUrlId::new(),
                })
            });

        mock_urls_repository
            .expect_create_short_url()
            .times(1)
            .returning(move |_| Ok(short_url.to_owned()));

        let service_under_test = Service::new(mock_urls_repository, service_config());

        let create_short_url_request = CreateShortUrlRequest::new(test_url.to_owned());
        let result = service_under_test
            .create_short_url(&create_short_url_request)
            .await
            .unwrap();
        let key = result.key();

        let result_short_url = result.short_url();
        let result_long_url = result.long_url();

        let expected_short_url =
            Url::parse(format!("{}{}", service_config().base_url, key).as_str()).unwrap();

        assert_eq!(result_long_url.as_str(), test_url.as_str());
        assert_eq!(result_short_url.as_str(), expected_short_url.as_str())
    }

    #[tokio::test]
    async fn get_short_url_should_return_none_when_not_found() {
        let test_url =
            Url::parse("https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust")
                .unwrap();
        let short_url = ShortUrl::from_long_url(&test_url).unwrap();

        let mut mock_urls_repository = MockMockUrlsRepository::new();

        mock_urls_repository
            .expect_find_short_url_by_key()
            .times(1)
            .returning(|_| Ok(None));

        let service_under_test = Service::new(mock_urls_repository, service_config());

        let maybe_short_url = service_under_test
            .retrieve_short_url(short_url.key().to_owned())
            .await
            .unwrap();

        assert_eq!(maybe_short_url, None)
    }

    #[tokio::test]
    async fn get_short_url_should_return_the_short_url_entity_when_found() {
        let test_url =
            Url::parse("https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust")
                .unwrap();
        let short_url = ShortUrl::from_long_url(&test_url).unwrap();

        let mut mock_urls_repository = MockMockUrlsRepository::new();

        let cloned_short_url = short_url.clone();
        mock_urls_repository
            .expect_find_short_url_by_key()
            .times(1)
            .return_once(move |_| Ok(Some(cloned_short_url)));

        let service_under_test = Service::new(mock_urls_repository, service_config());

        let result = service_under_test
            .retrieve_short_url(short_url.key().to_owned())
            .await
            .unwrap()
            .unwrap();

        let expected =
            ShortUrlResponse::from(short_url, service_config().base_url.to_owned()).unwrap();
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn delete_short_url_should_return_not_found_error() {
        let test_url =
            Url::parse("https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust")
                .unwrap();
        let short_url = ShortUrl::from_long_url(&test_url).unwrap();

        let mut mock_urls_repository = MockMockUrlsRepository::new();

        mock_urls_repository
            .expect_delete_short_url_by_key()
            .times(1)
            .return_once(move |_| Err(RepositoryShortUrlError::ShortUrlNotFound));

        let service_under_test = Service::new(mock_urls_repository, service_config());

        let result = service_under_test
            .delete_short_url(short_url.key().to_owned())
            .await
            .unwrap_err();

        assert_eq!(
            result.to_string(),
            RepositoryShortUrlError::ShortUrlNotFound.to_string()
        );
    }

    #[tokio::test]
    async fn delete_short_url_should_succeed() {
        let test_url =
            Url::parse("https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust")
                .unwrap();
        let short_url = ShortUrl::from_long_url(&test_url).unwrap();

        let mut mock_urls_repository = MockMockUrlsRepository::new();

        mock_urls_repository
            .expect_delete_short_url_by_key()
            .times(1)
            .return_once(move |_| Ok(()));

        let service_under_test = Service::new(mock_urls_repository, service_config());

        let result = service_under_test
            .delete_short_url(short_url.key().to_owned())
            .await
            .unwrap();

        assert_eq!(result, ());
    }
}
