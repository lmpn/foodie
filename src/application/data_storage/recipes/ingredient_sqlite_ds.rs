use async_trait::async_trait;
use sqlx::SqlitePool;
use tracing::error;

use crate::application::{
    domain::recipe::{ingredient::Ingredient, recipe::Recipe},
    ports::outgoing::recipe::{
        add_ingredient_port::{AddIngredientError, AddIngredientPort},
        delete_ingredient_port::{DeleteIngredientError, DeleteIngredientPort},
    },
};

impl From<sqlx::Error> for AddIngredientError {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value);
        match value {
            sqlx::Error::RowNotFound => AddIngredientError::RecordNotFound,
            _ => AddIngredientError::InternalError,
        }
    }
}

impl From<sqlx::Error> for DeleteIngredientError {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value);
        match value {
            sqlx::Error::RowNotFound => DeleteIngredientError::RecordNotFound,
            _ => DeleteIngredientError::InternalError,
        }
    }
}

#[derive(Debug, sqlx::FromRow, Clone)]
struct IngredientRecord {
    pub uuid: String,
    pub name: String,
    pub image: String,
    pub method: String,
}

impl Into<Recipe> for IngredientRecord {
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
pub struct IngredientSqliteDS {
    pool: SqlitePool,
}

impl IngredientSqliteDS {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AddIngredientPort for IngredientSqliteDS {
    async fn add_ingredient(
        &self,
        recipe_uuid: &str,
        ingredient: Ingredient,
    ) -> Result<(), AddIngredientError> {
        let uuid = ingredient.uuid().to_string();
        let unit = ingredient.unit();
        let name = ingredient.name();
        let amount = ingredient.amount();
        let transaction = self.pool.begin().await.unwrap();
        let result: Result<(), AddIngredientError> = sqlx::query!(
            r#" INSERT INTO ingredient (uuid, name, amount, unit, recipe_uuid) VALUES (?,?,?,?,?)"#,
            uuid,
            name,
            amount,
            unit,
            recipe_uuid
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(Into::into);
        if result.is_err() {
            transaction.rollback().await?;
            return result;
        }
        transaction.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl DeleteIngredientPort for IngredientSqliteDS {
    async fn delete_ingredient(
        &self,
        recipe_uuid: &str,
        ingredient_uuid: &str,
    ) -> Result<(), DeleteIngredientError> {
        sqlx::query!(
            r#" DELETE FROM ingredient WHERE uuid = ? AND recipe_uuid = ?"#,
            ingredient_uuid,
            recipe_uuid
        )
        .execute(&self.pool)
        .await
        .map(|_e| ())
        .map_err(Into::into)
    }
}
