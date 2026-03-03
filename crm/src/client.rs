use crm::pb::{RecallRequestBuilder, crm_client::CrmClient};
use anyhow::Result;
use tonic::Request;
use uuid::Uuid;



#[tokio::main]
async fn main() -> Result<()> {
    // let mut client = CrmClient::connect("http://[::1]:50000").await?;

    // let req = WelcomeRequestBuilder::default()
    //     .id(Uuid::new_v4().to_string())
    //     .interval(100u32)
    //     .content_ids([1u32, 2, 3])
    //     .build()?;
    // let response = client.welcome(Request::new(req)).await?;
    // println!("Response: {:?}", response);

    let mut clent = CrmClient::connect("http://[::1]:50000").await?;

    let req = RecallRequestBuilder::default()       
        .id(Uuid::new_v4().to_string())
        .interval(30u32)
        .content_ids(&[1, 2])
        .build()?;
    let response = clent.recall(Request::new(req)).await?;
    println!("Response: {:?}", response);
    Ok(())
}