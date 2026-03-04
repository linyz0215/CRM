use chrono::{DateTime, Utc};
use jwt_simple::prelude::*;
use tonic::{Request, Status, service::Interceptor};

#[derive(Debug, Clone)]
pub struct DecodingKey(Ed25519PublicKey);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";

impl DecodingKey {
    pub fn load(public_key: &str) -> Result<Self, jwt_simple::Error> {
        Ok(DecodingKey(Ed25519PublicKey::from_pem(public_key)?))
    }

    pub fn verify_token(&self, token: &str) -> Result<User, jwt_simple::Error> {
        let ops = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };
        let claims = self.0.verify_token::<User>(token, Some(ops))?;
        Ok(claims.custom)
    }
}

impl Interceptor for DecodingKey {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let token = req
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok());
        let user = match token {
            Some(bearer) => {
                let token = bearer
                    .strip_prefix("Bearer ")
                    .ok_or_else(|| Status::unauthenticated("invalid token format"))?;
                self.verify_token(token)
                    .map_err(|e| Status::unauthenticated(e.to_string()))?
            }
            None => return Err(Status::unauthenticated("missing token")),
        };

        req.extensions_mut().insert(user);
        Ok(req)
    }
}
