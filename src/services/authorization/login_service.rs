use crate::services::{
    domain::token_claims::TokenClaims,
    ports::{
        incoming::login_service::{LoginService, LoginServiceError},
        outgoing::query_user_by_email_port::{QueryUserByEmailError, QueryUserByEmailPort},
    },
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use jsonwebtoken::{EncodingKey, Header};
use tracing::error;

impl From<QueryUserByEmailError> for LoginServiceError {
    fn from(value: QueryUserByEmailError) -> Self {
        error!("{}", value);
        match value {
            QueryUserByEmailError::UserNotFound => LoginServiceError::InvalidCredentials,
            QueryUserByEmailError::InternalError => LoginServiceError::InternalError,
        }
    }
}

impl From<argon2::password_hash::Error> for LoginServiceError {
    fn from(value: argon2::password_hash::Error) -> Self {
        error!("{}", value);
        LoginServiceError::InvalidCredentials
    }
}

impl From<jsonwebtoken::errors::Error> for LoginServiceError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        error!("{:?}:{}", value.kind(), value);
        LoginServiceError::TokenEncodingError
    }
}

pub struct Login<Storage>
where
    Storage: QueryUserByEmailPort + Send + Sync,
{
    storage: Storage,
    secret: String,
    maxage: i64,
}

impl<Storage> Login<Storage>
where
    Storage: QueryUserByEmailPort + Send + Sync,
{
    pub fn new(storage: Storage, secret: String, maxage: i64) -> Self {
        Self {
            storage,
            secret,
            maxage,
        }
    }
}

#[async_trait]
impl<Storage> LoginService for Login<Storage>
where
    Storage: QueryUserByEmailPort + Send + Sync,
{
    async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<(String, i64), LoginServiceError> {
        let user = self.storage.query_user_by_email(email).await?;
        PasswordHash::new(user.password()).map(|parsed_hash| {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true)
        })?;
        let token = TokenClaims::new(user.id().to_string(), self.maxage);
        let token = jsonwebtoken::encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        Ok((token, self.maxage))
    }
}
