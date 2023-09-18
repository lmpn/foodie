use crate::application::{
    domain::authorization::token_claims::TokenClaims,
    ports::{
        incoming::authorization::login_command::{LoginCommand, LoginCommandError, Request},
        outgoing::authorization::query_user_by_email_port::{
            QueryUserByEmailError, QueryUserByEmailPort,
        },
    },
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use jsonwebtoken::{EncodingKey, Header};
use tracing::error;

impl From<QueryUserByEmailError> for LoginCommandError {
    fn from(value: QueryUserByEmailError) -> Self {
        error!("{}", value);
        match value {
            QueryUserByEmailError::UserNotFound => LoginCommandError::InvalidCredentials,
            QueryUserByEmailError::InternalError => LoginCommandError::InternalError,
        }
    }
}

impl From<argon2::password_hash::Error> for LoginCommandError {
    fn from(value: argon2::password_hash::Error) -> Self {
        error!("{}", value);
        LoginCommandError::InvalidCredentials
    }
}

impl From<jsonwebtoken::errors::Error> for LoginCommandError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        error!("{:?}:{}", value.kind(), value);
        LoginCommandError::TokenEncodingError
    }
}

pub struct LoginService<Storage>
where
    Storage: QueryUserByEmailPort + Send + Sync,
{
    storage: Storage,
    secret: String,
    maxage: i64,
}

impl<Storage> LoginService<Storage>
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
impl<Storage> LoginCommand for LoginService<Storage>
where
    Storage: QueryUserByEmailPort + Send + Sync,
{
    async fn login(&self, request: Request) -> Result<(String, i64), LoginCommandError> {
        let user = self.storage.query_user_by_email(request.email()).await?;
        PasswordHash::new(user.password()).map(|parsed_hash| {
            Argon2::default()
                .verify_password(request.password().as_bytes(), &parsed_hash)
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
