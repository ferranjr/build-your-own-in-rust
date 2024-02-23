use serde_aux::prelude::deserialize_number_from_string;
use std::net::SocketAddr;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub targets: Vec<SocketAddr>,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("load_balancer/configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("../configuration/base.yaml"),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
