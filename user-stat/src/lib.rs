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
    inner: Arc<UserStatsServiceInner>,
}

#[allow(dead_code)]
pub struct UserStatsServiceInner {
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
        let inner = UserStatsServiceInner { config, pool };
        Self {
            inner: Arc::new(inner),
        }
    }
    pub fn into_server(self) -> UserStatsServer<Self> {
        UserStatsServer::new(self)
    }
}

impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(feature = "test_utils")]
pub mod test_utils {
    use std::{path::PathBuf, sync::Arc};
    use sqlx::Executor;
    use chrono::{TimeZone, Utc};
    use prost_types::Timestamp;
    use sqlx::PgPool;
    use crate::{AppConfig, UserStatsService, UserStatsServiceInner, pb::{IdQuery, TimeQuery}};
    use anyhow::Result;
    use sqlx_db_tester::TestPg;

    impl UserStatsService {
        pub async fn new_for_test() -> Result<(TestPg, Self)> {
            let config = AppConfig::load()?;
            let post = config.server.db_url.rfind('/').expect("Invalid db_url");
            let server_url = &config.server.db_url[..post];
            let (tdb, pool) = get_test_pool(Some(server_url)).await;
            let svc = Self {
                inner: Arc::new(UserStatsServiceInner {
                    config,
                    pool: pool.clone(),
                }),
            };
            Ok((tdb, svc))
        }
    }

    pub async fn get_test_pool(server_url: Option<&str>) -> (TestPg, PgPool) {
        let url = match server_url {
            Some(url) => url.to_string(),
            None => format!("postgres://linyz@localhost:5432"),
        };
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let migrations = base.join("migrations");
        let tdb = TestPg::new(url, migrations);
        let pool = tdb.get_pool().await;
        let sql = include_str!("../fixtures/data.sql").split(";");
        let mut ts = pool.begin().await.expect("failed to begin transaction");

        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("failed to execute sql");
        }
        ts.commit().await.expect("failed to commit transaction");
        (tdb, pool)
    }



    pub fn id(id: &[u32]) -> IdQuery {
        IdQuery { ids: id.to_vec() }
    }

    pub fn tq(lower: Option<i64>, upper: Option<i64>) -> TimeQuery {
        TimeQuery {
            before: lower.map(to_ts),
            after: upper.map(to_ts),
        }
    }

    pub fn to_ts(days: i64) -> Timestamp {
        let dt = Utc
            .with_ymd_and_hms(2024, 5, 7, 0, 0, 0)
            .unwrap()
            .checked_sub_signed(chrono::Duration::days(days))
            .unwrap();
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}