use super::{
    domain::recipe::Recipe,
    ports::{
        incoming::update_recipe_service::{UpdateRecipeService, UpdateRecipeServiceError},
        outgoing::update_recipe_port::UpdateRecipePort,
    },
};
use async_trait::async_trait;

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
    async fn update_recipe(&self, _recipe: Recipe) -> Result<(), UpdateRecipeServiceError> {
        Ok(())
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
