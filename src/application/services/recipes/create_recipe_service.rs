use async_trait::async_trait;
use tracing::error;

use crate::application::ports::{
    incoming::recipe::create_recipe_command::{
        CreateRecipeCommand, CreateRecipeCommandError, Request,
    },
    outgoing::recipe::insert_recipe_port::{InsertRecipeError, InsertRecipePort},
};

impl From<InsertRecipeError> for CreateRecipeCommandError {
    fn from(value: InsertRecipeError) -> Self {
        error!("{}", value);
        match value {
            InsertRecipeError::InternalError => CreateRecipeCommandError::InternalError,
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
impl<Storage> CreateRecipeCommand for CreateRecipe<Storage>
where
    Storage: InsertRecipePort + Sync + Send,
{
    async fn create_recipe(
        &self,
        request: Request,
    ) -> Result<uuid::Uuid, CreateRecipeCommandError> {
        let uuid = uuid::Uuid::new_v4();
        let uuid_str = uuid.to_string();
        match self
            .storage
            .insert_recipe(&uuid_str, request.name(), request.image(), request.method())
            .await
        {
            Ok(()) => Ok(uuid),
            Err(InsertRecipeError::InternalError) => Err(CreateRecipeCommandError::InternalError),
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
