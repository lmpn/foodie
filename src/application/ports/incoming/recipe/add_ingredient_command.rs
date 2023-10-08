use async_trait::async_trait;
use std::{error::Error, fmt::Display};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Request {
    recipe_uuid: uuid::Uuid,
    name: String,
    amount: f64,
    unit: String,
}

impl Request {
    pub fn new(uuid: uuid::Uuid, name: String, amount: f64, unit: String) -> Self {
        Self {
            recipe_uuid: uuid,
            name,
            amount,
            unit,
        }
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

    pub fn uuid(&self) -> Uuid {
        self.recipe_uuid
    }
}

#[async_trait]
pub trait AddCommand {
    async fn add(&self, request: Request) -> Result<(), AddCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum AddCommandError {
    InternalError,
    RecipeNotFound,
}

impl Display for AddCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddCommandError::InternalError => f.write_str("Internal error"),
            AddCommandError::RecipeNotFound => f.write_str("Recipe not found"),
        }
    }
}
impl Error for AddCommandError {}
