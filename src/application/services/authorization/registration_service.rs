use crate::application::{
    domain::authorization::{filtered_user::FilteredUser, user::User},
    ports::{
        incoming::authorization::registration_service::{
            RegistrationService, RegistrationServiceError,
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

impl From<argon2::password_hash::Error> for RegistrationServiceError {
    fn from(value: argon2::password_hash::Error) -> Self {
        error!("{}", value);
        RegistrationServiceError::PasswordHash
    }
}

impl From<InsertUserError> for RegistrationServiceError {
    fn from(value: InsertUserError) -> Self {
        error!("{}", value);
        match value {
            InsertUserError::UserAlreadyExists => RegistrationServiceError::UserAlreadyExists,
            InsertUserError::InternalError => RegistrationServiceError::InternalError,
        }
    }
}

pub struct Registration<Storage>
where
    Storage: InsertUserPort + Send + Sync,
{
    storage: Storage,
}

impl<Storage> Registration<Storage>
where
    Storage: InsertUserPort + Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<Storage> RegistrationService for Registration<Storage>
where
    Storage: InsertUserPort + Send + Sync,
{
    async fn register(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<FilteredUser, RegistrationServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())?;

        let user = User::new(
            uuid::Uuid::new_v4(),
            name,
            email,
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
