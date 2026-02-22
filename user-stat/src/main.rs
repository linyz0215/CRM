use tonic::transport::Server;
use user_stat::UserStatsService;
use anyhow::Result;
use tracing::info;
use tracing_subscriber::{Layer as _, filter::LevelFilter, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};
use user_stat::config::AppConfig;




#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let config = AppConfig::load().expect("Failed to load config");
    let addr = config.server.port;
    let addr = format!("[::1]:{}", addr).parse().unwrap();
    info!("User stats service listening on {}", addr);

    let svc = UserStatsService::new(config).await.into_server();
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}