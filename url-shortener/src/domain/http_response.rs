#[derive(Debug, serde::Serialize)]
pub struct ShortenedUrlResponse<'a> {
    key: &'a str,
    long_url: &'a str,
    short_url: &'a str,
}