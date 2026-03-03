use crm_metadata::pb::metadata_client::MetadataClient;
use crm_send::pb::notification_client::NotificationClient;
use tonic::{Request, Response, Status, transport::Channel};
use user_stat::pb::user_stats_client::UserStatsClient;
use tonic::async_trait;
use crate::{config::AppConfig, pb::{RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse, crm_server::Crm}};

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
        self.welcome(request.into_inner()).await
    }
    ///
    async fn recall(
        &self,
        request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
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

    pub fn into_server(self) -> crate::pb::crm_server::CrmServer<Self> {
        crate::pb::crm_server::CrmServer::new(self)
    }
}