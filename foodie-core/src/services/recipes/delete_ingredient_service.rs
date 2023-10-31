use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use crate::ports::{
    incoming::recipe::delete_ingredient_command::{
        DeleteIngredientCommand, DeleteIngredientCommandError,
    },
    outgoing::recipe::delete_ingredient_port::{DeleteIngredientError, DeleteIngredientPort},
};

use super::service::RecipeService;

impl From<DeleteIngredientError> for DeleteIngredientCommandError {
    fn from(value: DeleteIngredientError) -> Self {
        error!("{}", value);
        match value {
            DeleteIngredientError::InternalError => DeleteIngredientCommandError::InternalError,
        }
    }
}

#[async_trait]
impl<Storage> DeleteIngredientCommand for RecipeService<Storage>
where
    Storage: DeleteIngredientPort + Send + Sync,
{
    async fn delete(
        &self,
        recipe_uuid: Uuid,
        ingredient_uuid: Uuid,
    ) -> Result<(), DeleteIngredientCommandError> {
        let recipe_uuid = recipe_uuid.to_string();
        let ingridient_uuid = ingredient_uuid.to_string();
        self.storage
            .delete_ingredient(&recipe_uuid, &ingridient_uuid)
            .await
            .map_err(|e| e.into())
    }
}
