use crate::services::{
    domain::recipe::Recipe,
    ports::{
        incoming::query_recipe_service::{QueryRecipeService, QueryRecipeServiceError},
        outgoing::query_recipe_port::{QueryRecipeError, QueryRecipePort},
    },
};
use async_trait::async_trait;
use tracing::error;

impl From<QueryRecipeError> for QueryRecipeServiceError {
    fn from(value: QueryRecipeError) -> Self {
        error!("{}", value);
        match value {
            QueryRecipeError::RecordNotFound => QueryRecipeServiceError::RecipeNotFound,
            QueryRecipeError::InternalError => QueryRecipeServiceError::InternalError,
        }
    }
}

pub struct QueryRecipe<Storage>
where
    Storage: QueryRecipePort + Send + Sync,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> QueryRecipeService for QueryRecipe<Storage>
where
    Storage: QueryRecipePort + Send + Sync,
{
    async fn query_recipe(&self, uuid: uuid::Uuid) -> Result<Recipe, QueryRecipeServiceError> {
        self.storage
            .query_recipe(uuid)
            .await
            .map_err(|err| err.into())
    }
}

impl<Storage> QueryRecipe<Storage>
where
    Storage: QueryRecipePort + Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
