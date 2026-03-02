use std::net::SocketAddr;

use anyhow::Result;
use crm::{CrmService, config::AppConfig};
use tonic::transport::Server;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};



#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let port = config.server.port;
    let addr: SocketAddr = format!("[::1]:{}", port).parse().unwrap();
    tracing::info!("CRM service listening on {}", addr);

    let svc = CrmService::try_new(config).await.into_server();
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
