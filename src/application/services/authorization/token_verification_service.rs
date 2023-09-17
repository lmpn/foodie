use crate::application::{
    domain::authorization::{token_claims::TokenClaims, user::User},
    ports::{
        incoming::authorization::token_verification_service::{
            TokenVerificationService, TokenVerificationServiceError,
        },
        outgoing::authorization::query_user_port::{QueryUserError, QueryUserPort},
    },
};
use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use tracing::error;

impl From<QueryUserError> for TokenVerificationServiceError {
    fn from(value: QueryUserError) -> Self {
        error!("{}", value);
        match value {
            QueryUserError::UserNotFound => TokenVerificationServiceError::UserNotFound,
            QueryUserError::InternalError => TokenVerificationServiceError::InternalError,
        }
    }
}
impl From<uuid::Error> for TokenVerificationServiceError {
    fn from(value: uuid::Error) -> Self {
        error!("{}", value);
        TokenVerificationServiceError::InternalError
    }
}

impl From<jsonwebtoken::errors::Error> for TokenVerificationServiceError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        error!("{:?}:{}", value.kind(), value);
        TokenVerificationServiceError::TokenDecoding
    }
}

pub struct TokenVerification<Storage>
where
    Storage: QueryUserPort + Send + Sync,
{
    storage: Storage,
    secret: String,
}

impl<Storage> TokenVerification<Storage>
where
    Storage: QueryUserPort + Send + Sync,
{
    pub fn new(storage: Storage, secret: String) -> Self {
        Self { storage, secret }
    }
}

#[async_trait]
impl<Storage> TokenVerificationService for TokenVerification<Storage>
where
    Storage: QueryUserPort + Send + Sync,
{
    async fn verify_token(&self, token: String) -> Result<User, TokenVerificationServiceError> {
        let claims = decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?
        .claims;
        let user_id = uuid::Uuid::parse_str(&claims.sub)?;
        self.storage.query_user(user_id).await.map_err(|e| e.into())
    }
}
