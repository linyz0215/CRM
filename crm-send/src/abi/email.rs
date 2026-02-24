use tonic::Status;
use tracing::warn;

use crate::{NotificationService, abi::{Sender, to_ts}, pb::{EmailMessage, SendRequest, SendResponse, send_request::Msg}};



impl Sender for EmailMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status>  {
      let message_id = self.message_id.clone();
        svc.sender.send(Msg::Email(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}



impl From<EmailMessage> for Msg {
    fn from(email: EmailMessage) -> Self {
        Msg::Email(email)
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(email: EmailMessage) -> Self {
        let msg: Msg = email.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl EmailMessage {
    pub fn fake() -> Self {
        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;
        use uuid::Uuid;
        EmailMessage {
            message_id: Uuid::new_v4().to_string(),
            sender: SafeEmail().fake(),
            recipients: vec![SafeEmail().fake()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}