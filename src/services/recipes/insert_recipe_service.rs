use crate::services::recipes::{
    domain::recipe::Recipe,
    ports::{
        incoming::insert_recipe_service::{InsertRecipeService, InsertRecipeServiceError},
        outgoing::insert_recipe_port::InsertRecipePort,
    },
};
use async_trait::async_trait;

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
    async fn insert_recipe(&self, _recipe: Recipe) -> Result<(), InsertRecipeServiceError> {
        Ok(())
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
