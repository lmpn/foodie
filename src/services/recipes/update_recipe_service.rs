use crate::services::{
    domain::recipe::Recipe,
    ports::{
        incoming::update_recipe_service::{UpdateRecipeService, UpdateRecipeServiceError},
        outgoing::update_recipe_port::{UpdateRecipeError, UpdateRecipePort},
    },
};
use async_trait::async_trait;
use uuid::Uuid;

impl From<UpdateRecipeError> for UpdateRecipeServiceError {
    fn from(value: UpdateRecipeError) -> Self {
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
    async fn update_recipe(
        &self,
        recipe: Recipe,
        deleted_ingredients: Vec<Uuid>,
    ) -> Result<(), UpdateRecipeServiceError> {
        self.storage
            .update_recipe(recipe, deleted_ingredients)
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
