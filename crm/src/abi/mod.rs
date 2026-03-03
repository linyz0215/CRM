use std::sync::Arc;

use chrono::{Duration, Utc};
use crm_metadata::pb::{Content, MaterializeRequest};

use crm_send::pb::SendRequest;
use futures::{StreamExt, stream};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::warn;
use user_stat::pb::QueryRequest;

use crate::{
    CrmService,
    pb::{RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse},
};

impl CrmService {
    pub async fn welcome(&self, req: WelcomeRequest) -> Result<Response<WelcomeResponse>, Status> {
        let request_id = req.id;
        let d1 = Utc::now() - Duration::days(req.interval as _);
        let d2 = d1 + Duration::days(1);
        let query = QueryRequest::new_with_dt("created_at", d1, d2);
        let mut res_user_stats = self.user_stats.clone().query(query).await?.into_inner();
        let stream = stream::iter(
            req.content_ids
                .into_iter()
                .map(|id| MaterializeRequest { id }),
        );
        let contents = self
            .metadata
            .clone()
            .materialize(stream)
            .await?
            .into_inner();
        let contents = contents
            .filter_map(|v| async move { v.ok() })
            .collect::<Vec<Content>>()
            .await;
        let contents = Arc::new(contents);

        let sender = self.config.server.sender_email.clone();
        let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            while let Some(Ok(user)) = res_user_stats.next().await {
                let contents = contents.clone();
                let sender = sender.clone();
                let tx = tx.clone();

                let req = SendRequest::new("Welcome".to_string(), sender, &[user.email], &contents);
                if let Err(e) = tx.send(req).await {
                    warn!("failed to send notification request: {}", e);
                }
            }
        });
        let reqs = ReceiverStream::new(rx);
        self.notification.clone().send(reqs).await?;
        Ok(Response::new(WelcomeResponse { id: request_id }))
    }

    pub async fn recall(&self, req: RecallRequest) -> Result<Response<RecallResponse>, Status> {
        let request_id = req.id;
        let d1 = Utc::now() - Duration::days(req.interval as _);
        let d2 = d1 + Duration::days(1);
        let query = QueryRequest::new_with_dt("last_watched_at", d1, d2);
        let mut res_user_stats = self.user_stats.clone().query(query).await?.into_inner();
        let stream = stream::iter(
            req.content_ids
                .into_iter()
                .map(|id| MaterializeRequest { id }),
        );
        let content = self
            .metadata
            .clone()
            .materialize(stream)
            .await?
            .into_inner();
        let content = content
            .filter_map(|v| async move { v.ok() })
            .collect::<Vec<_>>()
            .await;
        let content = Arc::new(content);
        let sender = self.config.server.sender_email.clone();
        let (tx, rx) = mpsc::channel(1024);

        tokio::spawn( async move {
            while let Some(Ok(user)) = res_user_stats.next().await {
                let req =  SendRequest::new("Record".to_string(), sender.clone(), &[user.email], &content);
                if let Err(e) = tx.send(req).await {
                    warn!("failed to send notification request: {}", e);    
                }
            }

        });
        let reqs = ReceiverStream::new(rx);
        self.notification.clone().send(reqs).await?;
        Ok(Response::new(RecallResponse { id: request_id }))
    }

    pub async fn remind(&self, req: RemindRequest) -> Result<Response<RemindResponse>, Status> {
        let _request_id = req.id;
        let d1 = Utc::now() - Duration::days(req.interval as _);
        let d2 = d1 + Duration::days(1);
        let _query = QueryRequest::new_with_dt("last_watched_at", d1, d2);
        //let mut res_user_stats = self.user_stats.clone().query(query).await?.into_inner();
        todo!()
    }





}
