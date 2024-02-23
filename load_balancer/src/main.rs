use load_balancer::configuration::get_configuration;
use load_balancer::domain::models::Targets;
use load_balancer::startup;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = get_configuration().expect("Unable to load configuration");
    let address = format!("{}:{}", "127.0.0.1", settings.application.port);
    let listener = TcpListener::bind(address).await?;
    let targets: Arc<Mutex<Targets>> =
        Arc::new(Mutex::new(Targets::new(settings.application.targets)));

    startup::run(listener, targets).await
}
