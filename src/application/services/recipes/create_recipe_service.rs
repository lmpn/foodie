use async_trait::async_trait;
use tracing::error;

use crate::application::ports::{
    incoming::recipe::create_command::{CreateCommand, CreateCommandError, Request},
    outgoing::recipe::insert_recipe_port::{InsertRecipeError, InsertRecipePort},
};

impl From<InsertRecipeError> for CreateCommandError {
    fn from(value: InsertRecipeError) -> Self {
        error!("{}", value);
        match value {
            InsertRecipeError::InternalError => CreateCommandError::InternalError,
        }
    }
}

pub struct CreateRecipe<Storage>
where
    Storage: InsertRecipePort + Sync + Send,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> CreateCommand for CreateRecipe<Storage>
where
    Storage: InsertRecipePort + Sync + Send,
{
    async fn insert(&self, request: Request) -> Result<(), CreateCommandError> {
        match self
            .storage
            .insert_recipe(
                uuid::Uuid::new_v4().to_string().as_str(),
                request.name(),
                request.image(),
                request.method(),
            )
            .await
        {
            Ok(()) => Ok(()),
            Err(InsertRecipeError::InternalError) => Err(CreateCommandError::InternalError),
        }
    }
}

impl<Storage> CreateRecipe<Storage>
where
    Storage: InsertRecipePort + Sync + Send,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
