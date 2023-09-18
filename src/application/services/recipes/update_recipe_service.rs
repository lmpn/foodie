use crate::application::ports::{
    incoming::recipe::update_command::{Request, UpdateCommand, UpdateCommandError},
    outgoing::recipe::update_recipe_port::{UpdateRecipeError, UpdateRecipePort},
};
use async_trait::async_trait;
use tracing::error;

impl From<UpdateRecipeError> for UpdateCommandError {
    fn from(value: UpdateRecipeError) -> Self {
        error!("{}", value);
        match value {
            UpdateRecipeError::RecordNotFound => UpdateCommandError::RecipeNotFound,
            UpdateRecipeError::InternalError => UpdateCommandError::InternalError,
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
impl<Storage> UpdateCommand for UpdateRecipe<Storage>
where
    Storage: UpdateRecipePort + Sync + Send,
{
    async fn update(&self, request: Request) -> Result<(), UpdateCommandError> {
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
