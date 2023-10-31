use super::service::RecipeService;
use crate::ports::{
    incoming::recipe::update_ingredient_command::{
        Request, UpdateIngredientCommand, UpdateIngredientCommandError,
    },
    outgoing::recipe::update_ingredient_port::{UpdateIngredientError, UpdateIngredientPort},
};
use async_trait::async_trait;
use tracing::error;

impl From<UpdateIngredientError> for UpdateIngredientCommandError {
    fn from(value: UpdateIngredientError) -> Self {
        error!("{}", value);
        match value {
            UpdateIngredientError::RecipeNotFound => UpdateIngredientCommandError::RecipeNotFound,
            UpdateIngredientError::InternalError => UpdateIngredientCommandError::InternalError,
        }
    }
}

#[async_trait]
impl<Storage> UpdateIngredientCommand for RecipeService<Storage>
where
    Storage: UpdateIngredientPort + Sync + Send,
{
    async fn update_ingredient(
        &self,
        request: Request,
    ) -> Result<(), UpdateIngredientCommandError> {
        self.storage
            .update_ingredient(
                request.uuid().to_string().as_str(),
                request.name(),
                request.amount(),
                request.unit(),
                request.recipe_uuid().to_string().as_str(),
            )
            .await
            .map_err(|e| e.into())
    }
}
