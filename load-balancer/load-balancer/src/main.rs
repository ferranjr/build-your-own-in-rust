use load_balancer::configuration::get_configuration;
use load_balancer::domain::models::Targets;
use load_balancer::healthchecks::healthchecker::HealthChecker;
use load_balancer::startup;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
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
