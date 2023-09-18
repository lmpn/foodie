use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct Request {
    name: String,
    image: String,
    method: String,
}

impl Request {
    pub fn new(name: String, image: String, method: String) -> Self {
        Self {
            name,
            image,
            method,
        }
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
pub trait CreateCommand {
    async fn insert(&self, recipe: Request) -> Result<(), CreateCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum CreateCommandError {
    InternalError,
}

impl Display for CreateCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateCommandError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for CreateCommandError {}
