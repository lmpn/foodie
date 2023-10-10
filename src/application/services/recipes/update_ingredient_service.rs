use crate::application::ports::{
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
            UpdateIngredientError::RecordNotFound => UpdateIngredientCommandError::RecipeNotFound,
            UpdateIngredientError::InternalError => UpdateIngredientCommandError::InternalError,
        }
    }
}

pub struct UpdateIngredient<Storage>
where
    Storage: UpdateIngredientPort + Sync + Send,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> UpdateIngredientCommand for UpdateIngredient<Storage>
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

impl<Storage> UpdateIngredient<Storage>
where
    Storage: UpdateIngredientPort + Sync + Send,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
