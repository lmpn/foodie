use async_trait::async_trait;
use std::{error::Error, fmt::Display};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Request {
    uuid: uuid::Uuid,
    name: String,
    amount: f64,
    unit: String,
    recipe_uuid: uuid::Uuid,
}

impl Request {
    pub fn new(
        uuid: uuid::Uuid,
        name: String,
        amount: f64,
        unit: String,
        recipe_uuid: uuid::Uuid,
    ) -> Self {
        Self {
            uuid,
            name,
            amount,
            unit,
            recipe_uuid,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn unit(&self) -> &str {
        self.unit.as_ref()
    }

    pub fn recipe_uuid(&self) -> Uuid {
        self.recipe_uuid
    }
}

#[async_trait]
pub trait UpdateIngredientCommand {
    async fn update_ingredient(&self, request: Request)
        -> Result<(), UpdateIngredientCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum UpdateIngredientCommandError {
    InternalError,
    RecipeNotFound,
    IngredientNotFound,
}

impl Display for UpdateIngredientCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateIngredientCommandError::InternalError => f.write_str("Internal error"),
            UpdateIngredientCommandError::RecipeNotFound => f.write_str("Recipe not found"),
            UpdateIngredientCommandError::IngredientNotFound => f.write_str("Ingredient not found"),
        }
    }
}
impl Error for UpdateIngredientCommandError {}
