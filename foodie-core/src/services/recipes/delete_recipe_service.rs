use async_trait::async_trait;
use tracing::error;

use crate::ports::{
    incoming::recipe::delete_recipe_command::{DeleteRecipeCommand, DeleteRecipeCommandError},
    outgoing::recipe::delete_recipe_port::{DeleteRecipeError, DeleteRecipePort},
};

use super::service::RecipeService;

impl From<DeleteRecipeError> for DeleteRecipeCommandError {
    fn from(value: DeleteRecipeError) -> Self {
        error!("{}", value);
        match value {
            DeleteRecipeError::InternalError => DeleteRecipeCommandError::InternalError,
            DeleteRecipeError::RecipeNotFound => DeleteRecipeCommandError::RecipeNotFound,
        }
    }
}

#[async_trait]
impl<Storage> DeleteRecipeCommand for RecipeService<Storage>
where
    Storage: DeleteRecipePort + Send + Sync,
{
    async fn delete_recipe(&self, uuid: uuid::Uuid) -> Result<(), DeleteRecipeCommandError> {
        self.storage
            .delete_recipe(uuid.to_string().as_str())
            .await
            .map_err(|e| e.into())
    }
}
