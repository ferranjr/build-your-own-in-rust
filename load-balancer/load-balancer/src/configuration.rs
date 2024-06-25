use serde;
use serde_aux::prelude::deserialize_number_from_string;
use std::env;
use std::net::SocketAddr;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub targets_dns: Vec<String>,
}

impl ApplicationSettings {
    pub async fn targets(&self) -> Vec<SocketAddr> {
        let mut addrs = Vec::new();
        for dns in &self.targets_dns {
            let mut host = tokio::net::lookup_host(dns)
                .await
                .expect("Failed to resolve DNS");
            if let Some(addr) = host.next() {
                addrs.push(addr);
            }
        }
        addrs
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("load-balancer/configuration");

    // Read the environment variable
    let config_file = env::var("CONFIG_FILE").unwrap_or_else(|_| String::from("base.yaml"));

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join(config_file),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
