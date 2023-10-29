use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use crate::{
    domain::recipe::ingredient::Ingredient,
    ports::{
        incoming::recipe::create_ingredient_command::{
            CreateIngredientCommand, CreateIngredientCommandError, Request,
        },
        outgoing::recipe::insert_ingredient_port::{InsertIngredientError, InsertIngredientPort},
    },
};

impl From<InsertIngredientError> for CreateIngredientCommandError {
    fn from(value: InsertIngredientError) -> Self {
        error!("{}", value);
        match value {
            InsertIngredientError::RecipeNotFound => CreateIngredientCommandError::RecipeNotFound,
            InsertIngredientError::InternalError => CreateIngredientCommandError::InternalError,
            InsertIngredientError::IngredientAlreadyExists => {
                CreateIngredientCommandError::IngredientAlreadyExists
            }
        }
    }
}

pub struct CreateIngredient<Storage>
where
    Storage: InsertIngredientPort + Send + Sync,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> CreateIngredientCommand for CreateIngredient<Storage>
where
    Storage: InsertIngredientPort + Send + Sync,
{
    async fn create_ingredient(
        &self,
        request: Request,
    ) -> Result<Uuid, CreateIngredientCommandError> {
        let uuid = Uuid::new_v4();
        let ingredient = Ingredient::new(
            uuid,
            request.name().to_string(),
            request.amount(),
            request.unit().to_string(),
        );
        let recipe_uuid = request.uuid().to_string();
        self.storage
            .insert_ingredient(&recipe_uuid, ingredient)
            .await
            .map(|_| uuid)
            .map_err(|e| e.into())
    }
}

impl<Storage> CreateIngredient<Storage>
where
    Storage: InsertIngredientPort + Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
