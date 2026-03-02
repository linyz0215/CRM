use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use crm_send::{AppConfig, NotificationService, pb::{EmailMessage, InAppMessage, SendRequest, SmsMessage, notification_client::NotificationClient, send_request::Msg}};
use tokio::time::sleep;
use tonic::{Request, transport::Server};
use tokio_stream::StreamExt;

#[tokio::test]
async fn test_send() -> Result<()> {
    let addr = start_server().await?;
    let mut client = NotificationClient::connect(format!("http://{}", addr)).await?;
    let stream = tokio_stream::iter(vec![
        SendRequest {
            msg: Some(Msg::Email(EmailMessage::fake())),
        },
        SendRequest {
            msg: Some(Msg::Sms(SmsMessage::fake())),
        },
        SendRequest {
            msg: Some(Msg::InApp(InAppMessage::fake())),
        },  
    ]);
    let request = Request::new(stream);
    let response = client.send(request).await?.into_inner();
    let ret: Vec<_> = response.map(|res| res.unwrap()).collect().await; 
    assert_eq!(ret.len(), 3);
    Ok(())
}






async fn start_server() -> Result<SocketAddr> {
    let config = AppConfig::load()?;
    let addr = format!("[::1]:{}", config.server.port).parse()?;
    let svc = NotificationService::new(config).into_server();

    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_secs(1)).await; // Wait for server to start
    Ok(addr)
}