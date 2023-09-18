use crate::application::{
    domain::recipe::recipe::Recipe,
    ports::{
        incoming::recipe::read_partial_query::{ReadPartialError, ReadPartialQuery},
        outgoing::recipe::query_recipe_port::{QueryRecipeError, QueryRecipePort},
    },
};
use async_trait::async_trait;
use tracing::error;

impl From<QueryRecipeError> for ReadPartialError {
    fn from(value: QueryRecipeError) -> Self {
        error!("{}", value);
        match value {
            QueryRecipeError::RecordNotFound => ReadPartialError::RecipeNotFound,
            QueryRecipeError::InternalError => ReadPartialError::InternalError,
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
impl<Storage> ReadPartialQuery for QueryRecipe<Storage>
where
    Storage: QueryRecipePort + Send + Sync,
{
    async fn read_recipe(&self, uuid: uuid::Uuid) -> Result<Recipe, ReadPartialError> {
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
