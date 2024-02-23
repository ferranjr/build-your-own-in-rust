use domain::models::Targets;
use load_balancer::configuration::get_configuration;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

mod domain;
mod proxy;
mod startup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = get_configuration().expect("Unable to load configuration");
    let address = format!("{}:{}", "127.0.0.1", settings.application.port);
    let listener = TcpListener::bind(address).await?;
    let targets: Arc<Mutex<Targets>> =
        Arc::new(Mutex::new(Targets::new(settings.application.targets)));

    startup::run(listener, targets).await
}
