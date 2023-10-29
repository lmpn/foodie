use crate::{
    domain::recipe::Recipe,
    ports::{
        incoming::recipe::recipe_query::{RecipeQuery, RecipeQueryError},
        outgoing::recipe::query_recipe_port::{QueryRecipeError, QueryRecipePort},
    },
};
use async_trait::async_trait;
use tracing::error;

impl From<QueryRecipeError> for RecipeQueryError {
    fn from(value: QueryRecipeError) -> Self {
        error!("{}", value);
        match value {
            QueryRecipeError::RecordNotFound => RecipeQueryError::RecipeNotFound,
            QueryRecipeError::InternalError => RecipeQueryError::InternalError,
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
impl<Storage> RecipeQuery for QueryRecipe<Storage>
where
    Storage: QueryRecipePort + Send + Sync,
{
    async fn recipe_query(&self, uuid: uuid::Uuid) -> Result<Recipe, RecipeQueryError> {
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
