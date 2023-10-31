use async_trait::async_trait;
use tracing::error;

use crate::ports::{
    incoming::recipe::create_recipe_command::{
        CreateRecipeCommand, CreateRecipeCommandError, Request,
    },
    outgoing::recipe::insert_recipe_port::{InsertRecipeError, InsertRecipePort},
};

use super::service::RecipeService;

impl From<InsertRecipeError> for CreateRecipeCommandError {
    fn from(value: InsertRecipeError) -> Self {
        error!("{}", value);
        match value {
            InsertRecipeError::InternalError => CreateRecipeCommandError::InternalError,
            InsertRecipeError::RecipeAlreadyExists => CreateRecipeCommandError::RecipeAlreadyExists,
        }
    }
}

#[async_trait]
impl<Storage> CreateRecipeCommand for RecipeService<Storage>
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
            Err(InsertRecipeError::RecipeAlreadyExists) => {
                Err(CreateRecipeCommandError::RecipeAlreadyExists)
            }
        }
    }
}
