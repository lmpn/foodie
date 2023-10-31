use super::service::RecipeService;
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

#[async_trait]
impl<Storage> RecipeQuery for RecipeService<Storage>
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
