use std::env;
use bson::doc;
use mongodb::{Client, Collection};
use crate::domain::database_models::ShortenedUrl;
use dotenv::dotenv;
use crate::repository::url_shortener_repository::UrlShortenerRepository;

pub struct MongoRepo {
    collection: Collection<ShortenedUrl>
}

impl MongoRepo {
    pub async fn init() -> std::io::Result<Self> {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable".to_string(),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("urlShortenerDb");
        let collection: Collection<ShortenedUrl> = db.collection("shortUrls");

        Ok(MongoRepo { collection })
    }
}

impl UrlShortenerRepository for MongoRepo {
    async fn get_shortened_url(&self, key: String) -> std::io::Result<Option<ShortenedUrl>> {
        let doc = doc! { "key": key };
        let result = self.collection
            .find_one(doc, None)
            .await
            .expect("Error getting shortened url details.");
        Ok(result)
    }

    async fn get_shortened_url_by_full_url(&self, full: &str) -> std::io::Result<Option<ShortenedUrl>> {
        let doc = doc! { "full": full };
        let result = self.collection
            .find_one(doc, None)
            .await
            .expect("Error getting the shortened url details.");
        Ok(result)
    }


    async fn save_shortened_url(
        &self,
        shortener_url: ShortenedUrl
    ) -> std::io::Result<ShortenedUrl> {
        self.collection.insert_one(&shortener_url, None)
            .await
            .expect("Failed to insert document in the database");

        Ok(shortener_url)
    }
}