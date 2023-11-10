use super::service::RecipeService;
use crate::{
    domain::recipe::Recipe,
    ports::{
        incoming::recipe::recipe_page_query::{RecipesPageQuery, RecipesPageQueryError},
        outgoing::recipe::query_recipes_port::{QueryRecipesError, QueryRecipesPort},
    },
};
use async_trait::async_trait;
use tracing::error;

impl From<QueryRecipesError> for RecipesPageQueryError {
    fn from(value: QueryRecipesError) -> Self {
        error!("{}", value);
        match value {
            QueryRecipesError::InternalError => RecipesPageQueryError::InternalError,
        }
    }
}
const MAX_RECIPES: u8 = 20;
#[async_trait]
impl<Storage> RecipesPageQuery for RecipeService<Storage>
where
Storage: QueryRecipesPort + Send + Sync,
{
    async fn recipes_page_query(
        &self,
        count: u8,
        offset: u8,
        ) -> Result<Vec<Recipe>, RecipesPageQueryError> {
        if count > MAX_RECIPES {
            return Err(RecipesPageQueryError::PageTooBig);
        }

        self.storage
            .query_recipes(count, offset)
            .await
            .map_err(|err| err.into())
    }
}
