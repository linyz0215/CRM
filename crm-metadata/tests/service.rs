use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use crm_metadata::{AppConfig, MetadataService, pb::{MaterializeRequest, metadata_client::MetadataClient}};
use tokio::time::sleep;
use tonic::{Request, transport::Server};
use tokio_stream::StreamExt;

#[tokio::test]
async fn test_metadata() -> Result<()> {
    let addr = start_server().await?;
    let mut client = MetadataClient::connect(format!("http://{}",addr)).await?;
    let stream = tokio_stream::iter([
        MaterializeRequest { id: 1},
        MaterializeRequest { id: 2},
        MaterializeRequest { id: 3},
    ]);
    let req = Request::new(stream);
    let response = client.materialize(req).await?.into_inner();
    let ret = response.map(|res| res.unwrap()).collect::<Vec<_>>().await;
    assert_eq!(ret.len(), 3);
    Ok(())
}






async fn start_server() -> Result<SocketAddr> {
    let config = AppConfig::load()?;
    let addr = format!("[::1]:{}", config.server.port).parse()?;

    let svc = MetadataService::new(config).into_server();
    tokio::spawn( async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_secs(1)).await; // Wait for server to start
    Ok(addr)
}