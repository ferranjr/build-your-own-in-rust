use std::env;

use anyhow::Context;

const MONGO_SHORT_URLS_DB: &str = "MONGO_SHORT_URLS_DB";
const MONGO_SHORT_URLS_DB_USERNAME: &str = "MONGO_SHORT_URLS_DB_USERNAME";
const MONGO_SHORT_URLS_DB_PASSWORD: &str = "MONGO_SHORT_URLS_DB_PASSWORD";
const MONGO_BASE_URL: &str = "MONGO_BASE_URL";
const MONGO_PORT: &str = "MONGO_PORT";
const SERVER_PORT: &str = "SERVER_PORT";
const SERVER_BASE_URL: &str = "SERVER_BASE_URL";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_base_url: String,
    pub server_port: u16,
    pub mongo_uri: String,
    pub mongo_database: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let server_port = load_env(SERVER_PORT)?.parse::<u16>()?;
        let server_base_url = load_env(SERVER_BASE_URL)?;
        let mongo_database = load_env(MONGO_SHORT_URLS_DB)?;
        let mongo_username = load_env(MONGO_SHORT_URLS_DB_USERNAME)?;
        let mongo_password = load_env(MONGO_SHORT_URLS_DB_PASSWORD)?;
        let mongo_base_url = load_env(MONGO_BASE_URL)?;
        let mongo_port = load_env(MONGO_PORT)?.parse::<u16>()?;
        let mongo_uri = format!(
            "mongodb://{}:{}@{}:{}/{}",
            mongo_username, mongo_password, mongo_base_url, mongo_port, mongo_database
        );

        Ok(Config {
            server_base_url,
            server_port,
            mongo_uri,
            mongo_database,
        })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("Missing environment variable {}", key))
}
