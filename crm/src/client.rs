use crm::pb::{RecallRequestBuilder, crm_client::CrmClient};
use anyhow::Result;
use tonic::{Request, metadata::MetadataValue, transport::{Certificate, Channel, ClientTlsConfig}};
use uuid::Uuid;



#[tokio::main]
async fn main() -> Result<()> {
    // let mut client = CrmClient::connect("http://127.0.0.1:8080").await?;
    // nginx 
    // let req = WelcomeRequestBuilder::default()
    //     .id(Uuid::new_v4().to_string())
    //     .interval(100u32)
    //     .content_ids([1u32, 2, 3])
    //     .build()?;
    // let response = client.welcome(Request::new(req)).await?;
    // println!("Response: {:?}", response);
    let pem = include_str!("../../fixtures/rootCA.pem");
    let tls = ClientTlsConfig::new().ca_certificate(Certificate::from_pem(pem)).domain_name("localhost");
    let channel = Channel::from_static("https://[::1]:50000").tls_config(tls)?.connect().await?;
    let token = include_str!("../../fixtures/token").trim();
    let token: MetadataValue<_> = format!("Bearer {}", token).parse()?;

    let mut client = CrmClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let req = RecallRequestBuilder::default()       
        .id(Uuid::new_v4().to_string())
        .interval(30u32)
        .content_ids(&[1, 2])
        .build()?;
    let response = client.recall(Request::new(req)).await?;
    println!("Response: {:?}", response);
    Ok(())
}