use mongodb::{Client, Collection, Database, bson::doc};
use tracing::instrument;
use url::Url;

use crate::domain::urls::{
    models::short_url::{RepositoryShortUrlError, ShortUrl, ShortUrlId},
    ports::UrlsRepository,
};

#[derive(Debug, Clone)]
pub struct MongoClient {
    client: Client,
}

impl MongoClient {
    pub async fn new(uri: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Self { client })
    }
}

#[derive(Debug, Clone)]
pub struct MongoDatabase {
    database: Database,
}

impl MongoDatabase {
    pub fn new(mongo_client: MongoClient, database_name: &str) -> MongoDatabase {
        Self {
            database: mongo_client.client.database(database_name),
        }
    }

    fn collection<T>(&self, name: &str) -> Collection<T>
    where
        T: Send + Sync,
    {
        self.database.collection::<T>(name)
    }

    fn short_urls_collection(&self) -> Collection<ShortUrl> {
        self.collection::<ShortUrl>("short_urls")
    }
}

impl UrlsRepository for MongoDatabase {
    #[instrument(skip(self))]
    async fn create_short_url(
        &self,
        short_url: ShortUrl,
    ) -> Result<ShortUrl, RepositoryShortUrlError> {
        let collection = self.short_urls_collection();
        let result = collection.insert_one(short_url).await?;

        collection
            .find_one(doc! { "_id": result.inserted_id })
            .await
            .map(|opt| match opt {
                // This shouldn't really be the case, as we just inserted it
                None => Err(RepositoryShortUrlError::ShortUrlNotFound),
                Some(v) => Ok(v),
            })?
    }

    #[instrument(skip(self))]
    async fn find_short_url_by_long_url(
        &self,
        long_url: Url,
    ) -> Result<Option<ShortUrl>, RepositoryShortUrlError> {
        let result = self
            .short_urls_collection()
            .find_one(doc! { "long_url": long_url.as_str() })
            .await?;

        Ok(result)
    }

    #[instrument(skip(self))]
    async fn find_short_url_by_short_url(
        &self,
        short_url: Url,
    ) -> Result<Option<ShortUrl>, RepositoryShortUrlError> {
        let result = self
            .short_urls_collection()
            .find_one(doc! { "short_url": short_url.as_str() })
            .await?;

        Ok(result)
    }

    #[instrument(skip(self))]
    async fn find_short_url_by_key(
        &self,
        key: ShortUrlId,
    ) -> Result<Option<ShortUrl>, RepositoryShortUrlError> {
        let result = self
            .short_urls_collection()
            .find_one(doc! { "key": key.as_ref() })
            .await?;

        Ok(result)
    }

    #[instrument(skip(self))]
    async fn delete_short_url_by_key(
        &self,
        key: ShortUrlId,
    ) -> Result<(), RepositoryShortUrlError> {
        let delete_result = self
            .short_urls_collection()
            .delete_one(doc! { "key": key.as_ref() })
            .await?;

        if delete_result.deleted_count > 0 {
            Ok(())
        } else {
            Err(RepositoryShortUrlError::ShortUrlNotFound)
        }
    }
}
