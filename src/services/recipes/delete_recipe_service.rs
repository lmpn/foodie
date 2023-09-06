use async_trait::async_trait;
use tracing::error;

use super::ports::{
    incoming::delete_recipe_service::{DeleteRecipeService, DeleteRecipeServiceError},
    outgoing::delete_recipe_port::DeleteRecipePort,
};

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
    async fn delete_recipe(&self, index: i64) -> Result<(), DeleteRecipeServiceError> {
        let path = match self.storage.delete_recipe(index).await {
            Ok(path) => path,
            Err(_) => return Err(DeleteRecipeServiceError::RecipeNotFound),
        };
        if std::fs::remove_file(&path).is_err() {
            error!("Error removing file {}", path);
            return Err(DeleteRecipeServiceError::InternalError);
        }
        Ok(())
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
