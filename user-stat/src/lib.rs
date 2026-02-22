use std::{ops::Deref, pin::Pin, sync::Arc};

use futures::Stream;
use sqlx::PgPool;
use tonic::{Request, Response, Status, async_trait};

use crate::{
    pb::{QueryRequest, RawQueryRequest, User, user_stats_server::{UserStats, UserStatsServer}},
};

pub mod pb;
mod abi;
pub mod config;

pub use config::AppConfig;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;
type ServiceResult<T> = Result<Response<T>, Status>;

#[derive(Clone)]
pub struct UserStatsService {
    inner: Arc<UserStateServiceInner>,
}

pub struct UserStateServiceInner {
    config: AppConfig,
    pool: PgPool,
}

#[async_trait]
impl UserStats for UserStatsService {
    type QueryStream = ResponseStream;
    type RawQueryStream = ResponseStream;
    async fn query(
        &self, 
        request: Request<QueryRequest>
    ) -> ServiceResult<Self::QueryStream> {
        let query = request.into_inner();
        self.query(query).await
    }
    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        let query = request.into_inner();
        self.raw_query(query).await
    }
}


impl UserStatsService {
    pub async fn new(config: AppConfig) -> Self {
        let pool = PgPool::connect(&config.server.db_url).await.unwrap();
        let inner = UserStateServiceInner { config, pool };
        Self {
            inner: Arc::new(inner),
        }
    }
    pub fn into_server(self) -> UserStatsServer<Self> {
        UserStatsServer::new(self)
    }
}

impl Deref for UserStatsService {
    type Target = UserStateServiceInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}