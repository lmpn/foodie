use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use crate::application::{
    domain::recipe::ingredient::Ingredient,
    ports::{
        incoming::recipe::add_ingredient_command::{
            AddIngredientCommand, AddIngredientCommandError, Request,
        },
        outgoing::recipe::add_ingredient_port::{AddIngredientError, AddIngredientPort},
    },
};

impl From<AddIngredientError> for AddIngredientCommandError {
    fn from(value: AddIngredientError) -> Self {
        error!("{}", value);
        match value {
            AddIngredientError::RecordNotFound => AddIngredientCommandError::RecipeNotFound,
            AddIngredientError::InternalError => AddIngredientCommandError::InternalError,
        }
    }
}

pub struct AddIngredient<Storage>
where
    Storage: AddIngredientPort + Send + Sync,
{
    storage: Storage,
}

#[async_trait]
impl<Storage> AddIngredientCommand for AddIngredient<Storage>
where
    Storage: AddIngredientPort + Send + Sync,
{
    async fn add_ingredient(&self, request: Request) -> Result<Uuid, AddIngredientCommandError> {
        let uuid = Uuid::new_v4();
        let ingredient = Ingredient::new(
            uuid,
            request.name().to_string(),
            request.amount(),
            request.unit().to_string(),
        );
        let recipe_uuid = request.uuid().to_string();
        self.storage
            .add_ingredient(&recipe_uuid, ingredient)
            .await
            .map(|_| uuid)
            .map_err(|e| e.into())
    }
}

impl<Storage> AddIngredient<Storage>
where
    Storage: AddIngredientPort + Send + Sync,
{
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
