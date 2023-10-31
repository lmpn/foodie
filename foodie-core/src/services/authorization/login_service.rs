use super::service::AuthorizationService;
use crate::{
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

#[async_trait]
impl<Storage> LoginCommand for AuthorizationService<Storage>
where
    Storage: QueryUserByEmailPort + Send + Sync,
{
    async fn login(&self, request: Request) -> Result<TokenClaims, LoginCommandError> {
        let user = self.storage.query_user_by_email(request.email()).await?;
        PasswordHash::new(user.password()).map(|parsed_hash| {
            Argon2::default()
                .verify_password(request.password().as_bytes(), &parsed_hash)
                .map_or(false, |_| true)
        })?;
        let token = TokenClaims::new(user.id().to_string(), self.maxage, "user".to_string());
        Ok(token)
    }
}
