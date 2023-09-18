use async_trait::async_trait;
use tracing::error;

use crate::application::ports::{
    incoming::recipe::delete_command::{DeleteCommand, DeleteCommandError},
    outgoing::recipe::delete_recipe_port::{DeleteRecipeError, DeleteRecipePort},
};

impl From<DeleteRecipeError> for DeleteCommandError {
    fn from(value: DeleteRecipeError) -> Self {
        error!("{}", value);
        match value {
            DeleteRecipeError::InternalError => DeleteCommandError::InternalError,
            DeleteRecipeError::RecordNotFound => DeleteCommandError::RecipeNotFound,
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
impl<Storage> DeleteCommand for DeleteRecipe<Storage>
where
    Storage: DeleteRecipePort + Send + Sync,
{
    async fn delete(&self, uuid: uuid::Uuid) -> Result<(), DeleteCommandError> {
        match self.storage.delete_recipe(uuid.to_string().as_str()).await {
            Err(DeleteRecipeError::RecordNotFound) => Err(DeleteCommandError::RecipeNotFound),
            Err(DeleteRecipeError::InternalError) => Err(DeleteCommandError::InternalError),
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
