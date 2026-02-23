mod config;
pub mod pb;
mod abi;
use std::pin::Pin;

pub use config::AppConfig;
use futures::Stream;
use tonic::{Request, Response, Status, Streaming, async_trait};

use crate::pb::{
    Content, MaterializeRequest, metadata_server::{Metadata, MetadataServer}
};

#[allow(dead_code)]
pub struct MetadataService {
    config: AppConfig,
}

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;
/*td::result::Result<
            tonic::Response<Self::MaterializeStream>,
            tonic::Status,
        >;
*/
#[async_trait]
impl Metadata for MetadataService {
    type MaterializeStream = ResponseStream;
    async fn materialize(
        &self, 
        request: Request<Streaming<MaterializeRequest>>
    ) -> ServiceResult<Self::MaterializeStream>{
        let request = request.into_inner();
        self.materialize(request).await
    }
}

impl MetadataService {
    pub fn new(config: AppConfig) -> Self {
        MetadataService { config }
    }
    pub fn into_server(self) -> MetadataServer<Self> {
        MetadataServer::new(self)
    }
}
