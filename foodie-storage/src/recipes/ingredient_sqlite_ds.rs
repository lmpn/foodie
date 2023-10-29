use async_trait::async_trait;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

use foodie_core::{
    domain::recipe::ingredient::Ingredient,
    ports::outgoing::recipe::{
        delete_ingredient_port::{DeleteIngredientError, DeleteIngredientPort},
        insert_ingredient_port::{InsertIngredientError, InsertIngredientPort},
        query_ingredients_port::{QueryIngredientsError, QueryIngredientsPort},
        update_ingredient_port::{UpdateIngredientError, UpdateIngredientPort},
    },
};

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
impl InsertIngredientPort for IngredientSqliteDS {
    async fn insert_ingredient(
        &self,
        recipe_uuid: &str,
        ingredient: Ingredient,
    ) -> Result<(), InsertIngredientError> {
        let uuid = ingredient.uuid().to_string();
        let unit = ingredient.unit();
        let name = ingredient.name();
        let amount = ingredient.amount();
        sqlx::query!(
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
        .map_err(|e| match e {
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    return InsertIngredientError::IngredientAlreadyExists;
                }
                if e.is_foreign_key_violation() {
                    return InsertIngredientError::RecipeNotFound;
                }
                InsertIngredientError::InternalError
            }
            _ => InsertIngredientError::InternalError,
        })
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
        .map(|_| ())
        .map_err(|_| DeleteIngredientError::InternalError)
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
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|_| QueryIngredientsError::InternalError)?;
        let recipe_found = sqlx::query!(r#"SELECT uuid FROM recipe WHERE uuid = ? "#, recipe_uuid)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => QueryIngredientsError::RecipeNotFound,
                _ => QueryIngredientsError::InternalError,
            });
        if recipe_found.is_err() {
            tx.rollback()
                .await
                .map_err(|_| QueryIngredientsError::InternalError)?;
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
        .map_err(|_| QueryIngredientsError::InternalError);
        if ingredients.is_err() {
            tx.rollback()
                .await
                .map_err(|_| QueryIngredientsError::InternalError)?;
        } else {
            tx.commit()
                .await
                .map_err(|_| QueryIngredientsError::InternalError)?;
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
        let result = sqlx::query!(
            r#"UPDATE ingredient SET name = ?, amount = ?, unit = ?  WHERE uuid = ? AND recipe_uuid = ?"#,
            name,
            amount,
        unit,
        uuid,
            recipe_uuid,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UpdateIngredientError::RecipeNotFound,
            _ => UpdateIngredientError::InternalError,
        })?;

        let e = result.rows_affected();
        if e == 0 {
            return Err(UpdateIngredientError::RecipeNotFound);
        }
        Ok(())
    }
}
