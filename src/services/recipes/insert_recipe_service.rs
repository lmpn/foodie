use async_trait::async_trait;
use tracing::error;

use crate::services::{
    domain::recipe::Recipe,
    ports::{
        incoming::insert_recipe_service::{InsertRecipeService, InsertRecipeServiceError},
        outgoing::insert_recipe_port::{InsertRecipeError, InsertRecipePort},
    },
};

impl From<InsertRecipeError> for InsertRecipeServiceError {
    fn from(value: InsertRecipeError) -> Self {
        error!("{}", value);
        match value {
            InsertRecipeError::InternalError => InsertRecipeServiceError::InternalError,
        }
    }
}

pub struct InsertRecipe<Storage>
where
    Storage: InsertRecipePort + Sync + Send,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> InsertRecipeService for InsertRecipe<Storage>
where
    Storage: InsertRecipePort + Sync + Send,
{
    async fn insert_recipe(&self, recipe: Recipe) -> Result<(), InsertRecipeServiceError> {
        if recipe.ingredients().is_empty() {
            return Err(InsertRecipeServiceError::NoIngredients);
        }
        match self.storage.insert_recipe(recipe).await {
            Ok(()) => Ok(()),
            Err(InsertRecipeError::InternalError) => Err(InsertRecipeServiceError::InternalError),
        }
    }
}

impl<Storage> InsertRecipe<Storage>
where
    Storage: InsertRecipePort + Sync + Send,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
