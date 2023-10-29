use crate::ports::{
    incoming::recipe::update_recipe_command::{
        Request, UpdateRecipeCommand, UpdateRecipeCommandError,
    },
    outgoing::recipe::update_recipe_port::{UpdateRecipeError, UpdateRecipePort},
};
use async_trait::async_trait;
use tracing::error;

impl From<UpdateRecipeError> for UpdateRecipeCommandError {
    fn from(value: UpdateRecipeError) -> Self {
        error!("{}", value);
        match value {
            UpdateRecipeError::RecordNotFound => UpdateRecipeCommandError::RecipeNotFound,
            UpdateRecipeError::InternalError => UpdateRecipeCommandError::InternalError,
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
impl<Storage> UpdateRecipeCommand for UpdateRecipe<Storage>
where
    Storage: UpdateRecipePort + Sync + Send,
{
    async fn update_recipe(&self, request: Request) -> Result<(), UpdateRecipeCommandError> {
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
