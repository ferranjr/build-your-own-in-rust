use std::sync::Arc;

use load_balancer::{
    configuration::get_configuration,
    domain::models::Targets,
    healthchecks::healthchecker::HealthChecker,
    startup,
};
use tokio::{net::TcpListener, sync::Mutex};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();
    let settings = get_configuration().expect("Unable to load configuration");
    let address = format!("{}:{}", "0.0.0.0", settings.application.port);
    let listener = TcpListener::bind(address).await?;
    let targets = settings.application.targets().await;
    let targets: Arc<Mutex<Targets>> = Arc::new(Mutex::new(Targets::new(targets)));

    // Initialises the monitoring of targets
    HealthChecker::init(Arc::clone(&targets)).await;

    // Initialise the server
    startup::run(listener, targets).await
}
