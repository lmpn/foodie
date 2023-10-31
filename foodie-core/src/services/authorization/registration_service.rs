use crate::{
    domain::authorization::{token_claims::TokenClaims, user::User},
    ports::{
        incoming::authorization::registration_command::{
            RegistrationCommand, RegistrationCommandError, Request,
        },
        outgoing::authorization::insert_user_port::{InsertUserError, InsertUserPort},
    },
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_trait::async_trait;
use tracing::error;

use super::service::AuthorizationService;

impl From<argon2::password_hash::Error> for RegistrationCommandError {
    fn from(value: argon2::password_hash::Error) -> Self {
        error!("{}", value);
        RegistrationCommandError::PasswordHash
    }
}

impl From<InsertUserError> for RegistrationCommandError {
    fn from(value: InsertUserError) -> Self {
        error!("{}", value);
        match value {
            InsertUserError::UserAlreadyExists => RegistrationCommandError::UserAlreadyExists,
            InsertUserError::InternalError => RegistrationCommandError::InternalError,
        }
    }
}

#[async_trait]
impl<Storage> RegistrationCommand for AuthorizationService<Storage>
where
    Storage: InsertUserPort + Send + Sync,
{
    async fn register(&self, request: Request) -> Result<TokenClaims, RegistrationCommandError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(request.password().as_bytes(), &salt)
            .map(|hash| hash.to_string())?;

        let user = User::new(
            uuid::Uuid::new_v4(),
            request.name().to_string(),
            request.email().to_string(),
            hashed_password,
            "user".to_string(),
            "default.png".to_string(),
            false,
        );
        let token = TokenClaims::new(user.id().to_string(), self.maxage, "user".to_string());
        self.storage.insert_user(user).await?;
        Ok(token)
    }
}
