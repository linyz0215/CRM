mod config;
mod abi;
pub mod pb;

use std::{pin::Pin, sync::Arc};

pub use config::AppConfig;
use futures::Stream;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming, async_trait};

use crate::pb::{SendRequest, SendResponse, notification_server::Notification, send_request::Msg};

#[derive(Clone)]
pub struct NotificationService {
    inner: Arc<NotificationServiceInner>,
}

#[allow(unused)]
pub struct NotificationServiceInner {
    config: AppConfig,
    sender: mpsc::Sender<Msg>,
}
//std::result::Result<tonic::Response<Self::SendStream>, tonic::Status>;
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;


#[async_trait]
impl Notification for NotificationService {
    type SendStream = ResponseStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> ServiceResult<Self::SendStream>{
        let stream = request.into_inner();
        self.send(stream).await
    }
}