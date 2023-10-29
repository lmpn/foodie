use crate::{
    domain::authorization::{filtered_user::FilteredUser, user::User},
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

pub struct RegistrationService<Storage>
where
    Storage: InsertUserPort + Send + Sync,
{
    storage: Storage,
}

impl<Storage> RegistrationService<Storage>
where
    Storage: InsertUserPort + Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<Storage> RegistrationCommand for RegistrationService<Storage>
where
    Storage: InsertUserPort + Send + Sync,
{
    async fn register(&self, request: Request) -> Result<FilteredUser, RegistrationCommandError> {
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
        let filtered_user = FilteredUser::new(
            user.id().to_owned(),
            user.name().to_owned(),
            user.email().to_owned(),
            user.role().to_owned(),
            user.photo().to_owned(),
            user.verified().to_owned(),
        );
        self.storage.insert_user(user).await?;
        Ok(filtered_user)
    }
}
