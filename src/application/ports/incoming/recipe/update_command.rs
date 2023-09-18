use async_trait::async_trait;
use std::{error::Error, fmt::Display};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Request {
    uuid: uuid::Uuid,
    name: String,
    image: String,
    method: String,
}

impl Request {
    pub fn new(uuid: uuid::Uuid, name: String, image: String, method: String) -> Self {
        Self {
            uuid,
            name,
            image,
            method,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn image(&self) -> &str {
        self.image.as_ref()
    }

    pub fn method(&self) -> &str {
        self.method.as_ref()
    }
}

#[async_trait]
pub trait UpdateCommand {
    async fn update(&self, request: Request) -> Result<(), UpdateCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum UpdateCommandError {
    InternalError,
    RecipeNotFound,
}

impl Display for UpdateCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateCommandError::InternalError => f.write_str("Internal error"),
            UpdateCommandError::RecipeNotFound => f.write_str("Recipe not found"),
        }
    }
}
impl Error for UpdateCommandError {}
