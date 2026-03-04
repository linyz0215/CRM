use crm_metadata::pb::metadata_client::MetadataClient;
use crm_send::pb::notification_client::NotificationClient;
use tonic::{Request, Response, Status, service::interceptor::InterceptedService, transport::Channel};
use tracing::info;
use user_stat::pb::user_stats_client::UserStatsClient;
use tonic::async_trait;
use crate::{ abi::{DecodingKey, User}, config::AppConfig, pb::{RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse, crm_server::{Crm, CrmServer}}};
use anyhow::Result;
pub mod config;
pub mod pb;
mod abi;

pub struct CrmService {
    config: AppConfig,
    user_stats: UserStatsClient<Channel>,
    notification: NotificationClient<Channel>,
    metadata: MetadataClient<Channel>,
}

#[async_trait]
impl Crm for CrmService {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        let user: &User = request.extensions().get().unwrap();
        self.welcome(request.into_inner()).await
    }
    ///
    async fn recall(
        &self,
        request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
        let user: &User = request.extensions().get().unwrap();
        info!("Recall request for user: {:?}", user);
        self.recall(request.into_inner()).await
    }
    ///
    async fn remind(
        &self,
        request: Request<RemindRequest>,
    ) -> Result<Response<RemindResponse>, Status> {
        self.remind(request.into_inner()).await
    }
}


impl CrmService {
    pub async fn try_new(config: AppConfig) -> Self {
        let user_stats = UserStatsClient::connect(config.server.user_stats.clone()).await.unwrap();
        let notification = NotificationClient::connect(config.server.notification.clone()).await.unwrap();
        let metadata = MetadataClient::connect(config.server.metadata.clone()).await.unwrap();
        Self {
            config,
            user_stats,
            notification,
            metadata,
        }
    }

    pub fn into_server(
        self,
    ) -> Result<InterceptedService<CrmServer<CrmService>, DecodingKey>> {
        let dk = DecodingKey::load(&self.config.auth.pk)?;
        Ok(CrmServer::with_interceptor(self, dk))
    }
}