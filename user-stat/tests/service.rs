use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use futures::StreamExt;
use sqlx_db_tester::TestPg;
use tokio::time::sleep;
use tonic::transport::Server;
use user_stat::{AppConfig, UserStatsService, pb::{QueryRequestBuilder,  RawQueryRequestBuilder, User, user_stats_client::UserStatsClient}, test_utils::{id, tq}};

const PORT_BASE: u32 = 60000;

#[tokio::test]
async fn query_should_work() -> Result<()> {
    let (tdb, addr) = start_server(PORT_BASE + 2).await?;
    let mut client = UserStatsClient::connect(format!("http://{}", addr)).await?;
    let query = QueryRequestBuilder::default()
        .timestamp(("created_at".to_string(), tq(Some(120), None)))
        .timestamp(("last_visited_at".to_string(), tq(Some(30), None)))
        .id(("viewed_but_not_started".to_string(), id(&[247631])))
        .build()
        .unwrap();
    let stream = client.query(query).await?.into_inner();
    let ret = stream.collect::<Vec<_>>().await;

    assert!(ret.len() > 0);
    Ok(())
}


#[tokio::test]
async fn raw_query_should_work() -> Result<()> {
    let (tdb, addr) = start_server(PORT_BASE + 1).await?;
    let mut client = UserStatsClient::connect(format!("http://{}", addr)).await?;
    let req = RawQueryRequestBuilder::default().query("SELECT email, name FROM user_stats WHERE created_at > '2024-01-01' LIMIT 5".to_string()).build()?;

    let stream = client.raw_query(req).await?.into_inner();
    let ret: Vec<User> = stream.map(|res| res.unwrap()).collect().await;
    assert_eq!(ret.len(), 5);
    Ok(())
}

async fn start_server(port: u32) -> Result<(TestPg, SocketAddr)> {
    let (tdb, svc) = UserStatsService::new_for_test().await?;
    let addr = format!("[::1]:{}", port).parse()?;
    tokio::spawn(async move {
        Server::builder()
            .add_service(svc.into_server())
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_secs(1)).await; // Wait for server to start
    Ok((tdb, addr))
}
