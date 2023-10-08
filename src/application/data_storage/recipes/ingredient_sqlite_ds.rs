use async_trait::async_trait;
use futures::TryFutureExt;
use sqlx::SqlitePool;
use tracing::error;

use crate::application::{
    domain::recipe::{ingredient::Ingredient, recipe::Recipe},
    ports::outgoing::recipe::{
        add_ingredient_port::{AddIngredientError, AddIngredientPort},
        remove_ingredient_port::{RemoveIngredientError, RemoveIngredientPort},
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

impl From<sqlx::Error> for RemoveIngredientError {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value);
        match value {
            sqlx::Error::RowNotFound => RemoveIngredientError::RecordNotFound,
            _ => RemoveIngredientError::InternalError,
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
            r#" INSERT INTO ingredient (uuid, name, amount, unit) VALUES (?,?,?,?)"#,
            uuid,
            name,
            amount,
            unit,
        )
        .execute(&self.pool)
        .and_then(|_| {
            sqlx::query!(
                r#" INSERT INTO recipe_ingredient (recipe_uuid, ingredient_uuid) VALUES (?,?)"#,
                recipe_uuid,
                uuid
            )
            .execute(&self.pool)
        })
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
impl RemoveIngredientPort for IngredientSqliteDS {
    async fn remove_ingredient(&self, uuid: &str) -> Result<(), RemoveIngredientError> {
        sqlx::query!(r#" DELETE FROM ingredient WHERE uuid = ?"#, uuid)
            .execute(&self.pool)
            .await
            .map(|_e| ())
            .map_err(Into::into)
    }
}
