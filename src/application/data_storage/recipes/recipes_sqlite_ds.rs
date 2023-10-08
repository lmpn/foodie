use async_trait::async_trait;
use sqlx::SqlitePool;
use tracing::info;

use crate::application::{
    domain::recipe::recipe::Recipe,
    ports::outgoing::recipe::{
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
    fn from(value: sqlx::Error) -> Self {
        info!("{}", value);
        InsertRecipeError::InternalError
    }
}

#[derive(Debug, sqlx::FromRow, Clone)]
struct RecipeRecord {
    pub uuid: String,
    pub name: String,
    pub image: String,
    pub method: String,
}

impl Into<Recipe> for RecipeRecord {
    fn into(self) -> Recipe {
        Recipe::new(
            uuid::Uuid::parse_str(&self.uuid).unwrap_or(uuid::Uuid::default()),
            self.name,
            self.image,
            self.method,
            vec![],
        )
    }
}

#[derive(Clone)]
pub struct RecipeSqliteDS {
    pool: SqlitePool,
}

impl RecipeSqliteDS {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UpdateRecipePort for RecipeSqliteDS {
    async fn update_recipe(
        &self,
        uuid: &str,
        name: &str,
        image: &str,
        method: &str,
    ) -> Result<(), UpdateRecipeError> {
        sqlx::query!(
            r#" UPDATE recipe SET name = ?, method = ?, image = ?  WHERE uuid = ?"#,
            name,
            method,
            image,
            uuid
        )
        .execute(&self.pool)
        .await
        .map(|_e| ())
        .map_err(Into::into)
    }
}

#[async_trait]
impl QueryRecipePort for RecipeSqliteDS {
    async fn query_recipe(&self, uuid: uuid::Uuid) -> Result<Recipe, QueryRecipeError> {
        let uuid = uuid.to_string();
        sqlx::query_as!(
            RecipeRecord,
            r#"SELECT * FROM recipe WHERE recipe.uuid = ?"#,
            uuid
        )
        .fetch_one(&self.pool)
        .await
        .map(Into::into)
        .map_err(Into::into)
    }
}

#[async_trait]
impl DeleteRecipePort for RecipeSqliteDS {
    async fn delete_recipe(&self, uuid: &str) -> Result<(), DeleteRecipeError> {
        sqlx::query!("DELETE FROM recipe WHERE uuid = ?", uuid)
            .execute(&self.pool)
            .await
            .map(|_v| ())
            .map_err(|e| e.into())
    }
}

#[async_trait]
impl InsertRecipePort for RecipeSqliteDS {
    async fn insert_recipe(
        &self,
        uuid: &str,
        name: &str,
        image: &str,
        method: &str,
    ) -> Result<(), InsertRecipeError> {
        sqlx::query!(
            "INSERT INTO recipe (uuid, name, method, image) VALUES (?, ?, ?, ?)",
            uuid,
            name,
            method,
            image
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
}
