use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use crate::application::ports::{
    incoming::recipe::delete_ingredient_command::{
        DeleteIngredientCommand, DeleteIngredientCommandError,
    },
    outgoing::recipe::delete_ingredient_port::{DeleteIngredientError, DeleteIngredientPort},
};

impl From<DeleteIngredientError> for DeleteIngredientCommandError {
    fn from(value: DeleteIngredientError) -> Self {
        error!("{}", value);
        match value {
            DeleteIngredientError::RecordNotFound => DeleteIngredientCommandError::RecipeNotFound,
            DeleteIngredientError::InternalError => DeleteIngredientCommandError::InternalError,
        }
    }
}

pub struct DeleteIngredient<Storage>
where
    Storage: DeleteIngredientPort + Send + Sync,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> DeleteIngredientCommand for DeleteIngredient<Storage>
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

impl<Storage> DeleteIngredient<Storage>
where
    Storage: DeleteIngredientPort + Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
