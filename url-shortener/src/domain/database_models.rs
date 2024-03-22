use nanoid::nanoid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ShortenedUrl {
    pub key: String,
    pub full: String,
    pub short: String,
}

impl ShortenedUrl {
    pub fn new(full: &str) -> ShortenedUrl {
        let key = nanoid!(6);
        let short = format!("http://localhost:8080/{}", key);

        let shortener_url: ShortenedUrl = ShortenedUrl {
            key: key.to_string(),
            full: full.to_string(),
            short: short.to_string()
        };

        shortener_url
    }
}

#[cfg(test)]
mod test {
    use nanoid::nanoid;
    use super::*;

    #[test]
    fn shortened_url_are_usable () {
        let key = nanoid!(6);
        let shortened = ShortenedUrl {
            key: key.clone(),
            full: "https://google.co.uk/".to_string(),
            short: "potato".to_string()
        };

        assert_eq!(shortened.key, key);
        assert_eq!(shortened.short, "potato");
        assert_eq!(shortened.full, "https://google.co.uk/");
    }
}