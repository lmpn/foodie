use async_trait::async_trait;
use tracing::error;

use crate::application::ports::{
    incoming::recipe::delete_recipe_command::{DeleteRecipeCommand, DeleteRecipeCommandError},
    outgoing::recipe::delete_recipe_port::{DeleteRecipeError, DeleteRecipePort},
};

impl From<DeleteRecipeError> for DeleteRecipeCommandError {
    fn from(value: DeleteRecipeError) -> Self {
        error!("{}", value);
        match value {
            DeleteRecipeError::InternalError => DeleteRecipeCommandError::InternalError,
            DeleteRecipeError::RecordNotFound => DeleteRecipeCommandError::RecipeNotFound,
        }
    }
}

pub struct DeleteRecipe<Storage>
where
    Storage: DeleteRecipePort + Send + Sync,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> DeleteRecipeCommand for DeleteRecipe<Storage>
where
    Storage: DeleteRecipePort + Send + Sync,
{
    async fn delete_recipe(&self, uuid: uuid::Uuid) -> Result<(), DeleteRecipeCommandError> {
        match self.storage.delete_recipe(uuid.to_string().as_str()).await {
            Err(DeleteRecipeError::RecordNotFound) => Err(DeleteRecipeCommandError::RecipeNotFound),
            Err(DeleteRecipeError::InternalError) => Err(DeleteRecipeCommandError::InternalError),
            _ => Ok(()),
        }
    }
}

impl<Storage> DeleteRecipe<Storage>
where
    Storage: DeleteRecipePort + Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
