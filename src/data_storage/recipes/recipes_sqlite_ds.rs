use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::services::recipes::{
    domain::recipe::Recipe,
    ports::outgoing::{
        delete_recipe_port::{DeleteRecipeError, DeleteRecipePort},
        insert_recipe_port::{InsertRecipeError, InsertRecipePort},
        query_recipe_port::{QueryRecipeError, QueryRecipePort},
        update_recipe_port::{UpdateRecipeError, UpdateRecipePort},
    },
};

impl From<sqlx::Error> for QueryRecipeError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => QueryRecipeError::RecordNotFound,
            _ => QueryRecipeError::InternalError,
        }
    }
}

impl From<sqlx::Error> for UpdateRecipeError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => UpdateRecipeError::RecordNotFound,
            _ => UpdateRecipeError::InternalError,
        }
    }
}

impl From<sqlx::Error> for DeleteRecipeError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => DeleteRecipeError::RecordNotFound,
            _ => DeleteRecipeError::InternalError,
        }
    }
}

impl From<sqlx::Error> for InsertRecipeError {
    fn from(_value: sqlx::Error) -> Self {
        InsertRecipeError::InternalError
    }
}

#[derive(Clone)]
pub struct RecipeSqliteDS {
    pool: SqlitePool,
}

#[async_trait]
impl UpdateRecipePort for RecipeSqliteDS {
    async fn update_recipe(&self, record: Recipe) -> Result<(), UpdateRecipeError> {
        Err(UpdateRecipeError::InternalError)
    }
}

#[async_trait]
impl QueryRecipePort for RecipeSqliteDS {
    async fn query_recipe(&self, index: i64) -> Result<Recipe, QueryRecipeError> {
        Err(QueryRecipeError::InternalError)
    }
}

#[async_trait]
impl DeleteRecipePort for RecipeSqliteDS {
    async fn delete_recipe(&self, index: i64) -> Result<String, DeleteRecipeError> {
        Err(DeleteRecipeError::InternalError)
    }
}

#[async_trait]
impl InsertRecipePort for RecipeSqliteDS {
    async fn insert_recipe(&self, record: &Recipe) -> Result<(), InsertRecipeError> {
        Err(InsertRecipeError::InternalError)
    }
}

impl RecipeSqliteDS {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
