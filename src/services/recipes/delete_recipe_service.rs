use async_trait::async_trait;
use tracing::error;

use crate::services::ports::{
    incoming::delete_recipe_service::{DeleteRecipeService, DeleteRecipeServiceError},
    outgoing::delete_recipe_port::{DeleteRecipeError, DeleteRecipePort},
};

impl From<DeleteRecipeError> for DeleteRecipeServiceError {
    fn from(value: DeleteRecipeError) -> Self {
        error!("{}", value);
        match value {
            DeleteRecipeError::InternalError => DeleteRecipeServiceError::InternalError,
            DeleteRecipeError::RecordNotFound => DeleteRecipeServiceError::RecipeNotFound,
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
impl<Storage> DeleteRecipeService for DeleteRecipe<Storage>
where
    Storage: DeleteRecipePort + Send + Sync,
{
    async fn delete_recipe(&self, uuid: uuid::Uuid) -> Result<(), DeleteRecipeServiceError> {
        match self.storage.delete_recipe(uuid).await {
            Err(DeleteRecipeError::RecordNotFound) => Err(DeleteRecipeServiceError::RecipeNotFound),
            Err(DeleteRecipeError::InternalError) => Err(DeleteRecipeServiceError::InternalError),
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
