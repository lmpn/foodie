use async_trait::async_trait;
use tracing::error;

use crate::{
    domain::recipe::ingredient::Ingredient,
    ports::{
        incoming::recipe::ingredient_page_query::{
            IngredientsPageQuery, IngredientsPageQueryError,
        },
        outgoing::recipe::query_ingredients_port::{QueryIngredientsError, QueryIngredientsPort},
    },
};

impl From<QueryIngredientsError> for IngredientsPageQueryError {
    fn from(value: QueryIngredientsError) -> Self {
        error!("{}", value);
        match value {
            QueryIngredientsError::RecipeNotFound => IngredientsPageQueryError::RecipeNotFound,
            QueryIngredientsError::InternalError => IngredientsPageQueryError::InternalError,
        }
    }
}

pub struct QueryIngredientsPage<Storage>
where
    Storage: QueryIngredientsPort + Send + Sync,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> IngredientsPageQuery for QueryIngredientsPage<Storage>
where
    Storage: QueryIngredientsPort + Send + Sync,
{
    async fn ingredients_page_query(
        &self,
        recipe_uuid: uuid::Uuid,
        count: i64,
        offset: i64,
    ) -> Result<Vec<Ingredient>, IngredientsPageQueryError> {
        let recipe_uuid = recipe_uuid.to_string();
        self.storage
            .query_ingredients(&recipe_uuid, count, offset)
            .await
            .map_err(|err| err.into())
    }
}

impl<Storage> QueryIngredientsPage<Storage>
where
    Storage: QueryIngredientsPort + Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}