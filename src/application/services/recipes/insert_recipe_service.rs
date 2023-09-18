use async_trait::async_trait;
use tracing::error;

use crate::application::ports::{
    incoming::recipe::insert_recipe_service::{
        InsertRecipeService, InsertRecipeServiceError, Request,
    },
    outgoing::recipe::insert_recipe_port::{InsertRecipeError, InsertRecipePort},
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
    async fn insert_recipe(&self, request: Request) -> Result<(), InsertRecipeServiceError> {
        match self
            .storage
            .insert_recipe(
                uuid::Uuid::new_v4().to_string().as_str(),
                request.name(),
                request.image(),
                request.method(),
            )
            .await
        {
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
