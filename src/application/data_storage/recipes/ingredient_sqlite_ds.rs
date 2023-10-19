use async_trait::async_trait;
use sqlx::SqlitePool;
use tracing::error;

use crate::application::{
    domain::recipe::ingredient::Ingredient,
    ports::outgoing::recipe::{
        add_ingredient_port::{AddIngredientError, AddIngredientPort},
        delete_ingredient_port::{DeleteIngredientError, DeleteIngredientPort},
        query_ingredients_port::{QueryIngredientsError, QueryIngredientsPort},
        update_ingredient_port::{UpdateIngredientError, UpdateIngredientPort},
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
            _ => DeleteIngredientError::InternalError,
        }
    }
}

impl From<sqlx::Error> for QueryIngredientsError {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value);
        match value {
            sqlx::Error::RowNotFound => QueryIngredientsError::RecipeNotFound,
            _ => QueryIngredientsError::InternalError,
        }
    }
}

impl From<sqlx::Error> for UpdateIngredientError {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value);
        match value {
            sqlx::Error::RowNotFound => UpdateIngredientError::RecordNotFound,
            _ => UpdateIngredientError::InternalError,
        }
    }
}
#[derive(Debug, sqlx::FromRow, Clone)]
struct IngredientRecord {
    pub uuid: String,
    pub name: String,
    pub amount: f64,
    pub unit: String,
}

impl Into<Ingredient> for IngredientRecord {
    fn into(self) -> Ingredient {
        Ingredient::new(
            uuid::Uuid::parse_str(&self.uuid).unwrap_or(uuid::Uuid::default()),
            self.name,
            self.amount,
            self.unit,
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

#[async_trait]
impl QueryIngredientsPort for IngredientSqliteDS {
    async fn query_ingredients(
        &self,
        recipe_uuid: &str,
        count: i64,
        offset: i64,
    ) -> Result<Vec<Ingredient>, QueryIngredientsError> {
        let tx = self.pool.begin().await?;
        let recipe_found = sqlx::query!(r#"SELECT uuid FROM recipe WHERE uuid = ? "#, recipe_uuid)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::<QueryIngredientsError>::into);
        if recipe_found.is_err() {
            tx.rollback()
                .await
                .map_err(Into::<QueryIngredientsError>::into)?;
            return Err(recipe_found.unwrap_err());
        }
        let ingredients = sqlx::query_as!(
            IngredientRecord,
            r#"SELECT uuid, name, amount, unit FROM ingredient WHERE recipe_uuid = ? ORDER BY uuid ASC LIMIT ? OFFSET ? "#,
            recipe_uuid,
            count,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map(|e| e.into_iter().map(Into::into).collect())
        .map_err(Into::into);
        if ingredients.is_err() {
            tx.rollback()
                .await
                .map_err(Into::<QueryIngredientsError>::into)?;
        } else {
            tx.commit()
                .await
                .map_err(Into::<QueryIngredientsError>::into)?;
        }
        return ingredients;
    }
}

#[async_trait]
impl UpdateIngredientPort for IngredientSqliteDS {
    async fn update_ingredient(
        &self,
        uuid: &str,
        name: &str,
        amount: f64,
        unit: &str,
        recipe_uuid: &str,
    ) -> Result<(), UpdateIngredientError> {
        let recipe_uuid = recipe_uuid.to_string();
        sqlx::query!(
            r#"UPDATE ingredient SET name = ?, amount = ?, unit = ?  WHERE uuid = ? AND recipe_uuid = ?"#,
            name,
            amount,
        unit,
        uuid,
            recipe_uuid,
        )
        .execute(&self.pool)
        .await
        .map(|_e| ())
        .map_err(Into::into)
    }
}
