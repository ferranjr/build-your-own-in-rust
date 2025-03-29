use std::fmt::{Display, Formatter};

use mongodb::error::Error;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortUrl {
    key: ShortUrlId,
    long_url: Url,
}

impl ShortUrl {
    pub fn from_long_url(long_url: &Url) -> Result<Self, ParseError> {
        let short_url_id = ShortUrlId::new();
        Self::new(short_url_id, long_url.to_owned())
    }

    pub fn new(key: ShortUrlId, long_url: Url) -> Result<Self, ParseError> {
        Ok(Self { key, long_url })
    }

    pub fn key(&self) -> &ShortUrlId {
        &self.key
    }

    pub fn long_url(&self) -> &Url {
        &self.long_url
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortUrlId(String);

impl ShortUrlId {
    pub fn new() -> Self {
        Self(nanoid!(6).to_string())
    }
}

impl Default for ShortUrlId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ShortUrlId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ShortUrlId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateShortUrlRequest {
    long_url: Url,
}

impl CreateShortUrlRequest {
    pub fn new(long_url: Url) -> Self {
        Self { long_url }
    }

    pub fn long_url(&self) -> &Url {
        &self.long_url
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortUrlResponse {
    key: ShortUrlId,
    long_url: Url,
    short_url: Url,
}

impl ShortUrlResponse {
    pub fn new(key: ShortUrlId, long_url: Url, short_url: Url) -> ShortUrlResponse {
        Self {
            key,
            long_url,
            short_url,
        }
    }

    pub fn from(short_url: ShortUrl, base_url: String) -> Result<ShortUrlResponse, ParseError> {
        let shortened_url = Url::parse(format!("{base_url}{}", short_url.key.0).as_str())?;
        Ok(Self {
            key: short_url.key,
            long_url: short_url.long_url,
            short_url: shortened_url,
        })
    }

    pub fn key(&self) -> &ShortUrlId {
        &self.key
    }

    pub fn long_url(&self) -> &Url {
        &self.long_url
    }

    pub fn short_url(&self) -> &Url {
        &self.short_url
    }
}

#[derive(Debug, Error)]
pub enum RepositoryShortUrlError {
    #[error("short_url with id {short_url_id} already exists")]
    Duplicate { short_url_id: ShortUrlId },
    #[error("short_url malformed")]
    UrlBadlyFormatted { parse_error: ParseError },
    #[error("short_url not found")]
    ShortUrlNotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    // to be extended as new error scenarios are introduced
}

impl From<ParseError> for RepositoryShortUrlError {
    fn from(parse_error: ParseError) -> Self {
        RepositoryShortUrlError::UrlBadlyFormatted { parse_error }
    }
}

impl From<mongodb::error::Error> for RepositoryShortUrlError {
    fn from(value: Error) -> Self {
        RepositoryShortUrlError::Unknown(anyhow::anyhow!(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_key_should_create_different_ones() {
        let url1 = ShortUrlId::new();
        let url2 = ShortUrlId::new();
        let url3 = ShortUrlId::new();
        let url4 = ShortUrlId::new();

        assert_ne!(url1, url2);
        assert_ne!(url2, url3);
        assert_ne!(url1, url3);
        assert_ne!(url1, url4);
        assert_ne!(url2, url4);
        assert_ne!(url3, url4);
    }

    #[test]
    fn url_key_should_be_of_6_characters() {
        let url = ShortUrlId::new();
        assert_eq!(url.0.len(), 6);
    }

    #[test]
    fn create_short_url_entity_from_just_url() {
        let long_url =
            Url::parse("https://codingchallenges.fyi/challenges/challenge-url-shortener/").unwrap();
        let short_url = ShortUrl::from_long_url(&long_url).unwrap();

        assert_eq!(short_url.long_url().as_str(), long_url.as_str())
    }
}
