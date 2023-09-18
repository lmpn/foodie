use crate::application::ports::{
    incoming::recipe::update_recipe_service::{
        Request, UpdateRecipeService, UpdateRecipeServiceError,
    },
    outgoing::recipe::update_recipe_port::{UpdateRecipeError, UpdateRecipePort},
};
use async_trait::async_trait;
use tracing::error;

impl From<UpdateRecipeError> for UpdateRecipeServiceError {
    fn from(value: UpdateRecipeError) -> Self {
        error!("{}", value);
        match value {
            UpdateRecipeError::RecordNotFound => UpdateRecipeServiceError::RecipeNotFound,
            UpdateRecipeError::InternalError => UpdateRecipeServiceError::InternalError,
        }
    }
}

pub struct UpdateRecipe<Storage>
where
    Storage: UpdateRecipePort + Sync + Send,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> UpdateRecipeService for UpdateRecipe<Storage>
where
    Storage: UpdateRecipePort + Sync + Send,
{
    async fn update_recipe(&self, request: Request) -> Result<(), UpdateRecipeServiceError> {
        self.storage
            .update_recipe(
                request.uuid().to_string().as_str(),
                request.name(),
                request.image(),
                request.method(),
            )
            .await
            .map_err(|e| e.into())
    }
}

impl<Storage> UpdateRecipe<Storage>
where
    Storage: UpdateRecipePort + Sync + Send,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
