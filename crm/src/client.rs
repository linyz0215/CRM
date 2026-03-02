use crm::pb::{WelcomeRequestBuilder, crm_client::CrmClient};
use anyhow::Result;
use tonic::Request;
use uuid::Uuid;



#[tokio::main]
async fn main() -> Result<()> {
    let mut client = CrmClient::connect("http://[::1]:50000").await?;

    let req = WelcomeRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .interval(100u32)
        .content_ids([1u32, 2, 3])
        .build()?;
    let response = client.welcome(Request::new(req)).await?;
    println!("Response: {:?}", response);
    Ok(())
}