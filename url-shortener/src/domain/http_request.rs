#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateShortUrl {
    pub url: String
}