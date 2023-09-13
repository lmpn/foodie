use async_trait::async_trait;
use futures::TryFutureExt;
use sqlx::{QueryBuilder, SqlitePool};
use tracing::info;
use uuid::Uuid;

use crate::services::recipes::{
    domain::{ingredient::Ingredient, recipe::Recipe},
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
    fn from(value: sqlx::Error) -> Self {
        info!("{}", value);
        InsertRecipeError::InternalError
    }
}

#[derive(Clone)]
pub struct RecipeSqliteDS {
    pool: SqlitePool,
}

#[async_trait]
impl UpdateRecipePort for RecipeSqliteDS {
    async fn update_recipe(
        &self,
        record: Recipe,
        deleted_ingredients: Vec<uuid::Uuid>,
    ) -> Result<(), UpdateRecipeError> {
        let transaction = self.pool.begin().await?;
        let name = record.name().to_string();
        let method = record.method();
        let image = record.image();
        let uuid = record.uuid().to_string();
        let result = sqlx::query!(
            r#" UPDATE recipe SET name = ?, method = ?, image = ?  WHERE uuid = ?  "#,
            name,
            method,
            image,
            uuid
        )
        .execute(&self.pool)
        .await
        .map(|_e| ())
        .map_err(|e| e.into());
        if result.is_err() {
            transaction.rollback().await?;
            return result;
        }
        for uuid in deleted_ingredients {
            let uuid = uuid.to_string();
            let result = sqlx::query!(r#"DELETE FROM ingredient WHERE uuid = ?"#, uuid)
                .execute(&self.pool)
                .await
                .map(|_e| ())
                .map_err(|e| e.into());
            if result.is_err() {
                transaction.rollback().await?;
                return result;
            }
        }
        for item in record.ingredients() {
            let uuid = item.uuid().to_string();
            let name = item.name();
            let amount = item.amount();
            let unit = item.unit();
            let result = sqlx::query!(
                "INSERT INTO ingredient (uuid, name, amount, unit) 
                          VALUES (?,?,?,?) 
                          ON CONFLICT (uuid) 
                          DO UPDATE SET name = ?, amount = ?, unit = ?",
                uuid,
                name,
                amount,
                unit,
                name,
                amount,
                unit
            )
            .execute(&self.pool)
            .await
            .map(|_e| ())
            .map_err(|e| e.into());
            if result.is_err() {
                transaction.rollback().await?;
                return result;
            }
        }
        transaction.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl QueryRecipePort for RecipeSqliteDS {
    async fn query_recipe(&self, uuid: uuid::Uuid) -> Result<Recipe, QueryRecipeError> {
        let uuid = uuid.to_string();
        let result = sqlx::query!(
            r#"SELECT recipe.uuid as ruuid, recipe.name as rname, recipe.method, recipe.image, ingredient.uuid, ingredient.name, ingredient.unit, ingredient.amount FROM recipe 
            JOIN recipe_ingredient ON recipe.uuid = recipe_uuid
            JOIN ingredient ON ingredient.uuid = ingredient_uuid 
            WHERE recipe.uuid = ?"#,
            uuid
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into());
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let records = result.unwrap();
        if records.is_empty() {
            return Err(QueryRecipeError::RecordNotFound);
        }

        let ingredient = records
            .iter()
            .map(|record| {
                Ingredient::new(
                    Uuid::parse_str(record.uuid.as_ref().unwrap().as_str()).unwrap(),
                    record.name.clone(),
                    record.amount,
                    record.unit.clone(),
                )
            })
            .collect::<Vec<Ingredient>>();
        let row = records.get(0).unwrap();
        let recipe = Recipe::new(
            Uuid::parse_str(
                row.uuid
                    .as_ref()
                    .ok_or(QueryRecipeError::InternalError)?
                    .as_str(),
            )
            .map_err(|_e| QueryRecipeError::InternalError)?,
            row.rname.clone(),
            row.image.clone().unwrap_or(String::new()),
            row.method.clone().unwrap_or(String::new()),
            ingredient,
        );
        Ok(recipe)
    }
}

#[async_trait]
impl DeleteRecipePort for RecipeSqliteDS {
    async fn delete_recipe(&self, uuid: Uuid) -> Result<(), DeleteRecipeError> {
        let mut builder = QueryBuilder::new("DELETE FROM recipe WHERE uuid = ");
        let query = builder.push_bind(uuid.to_string()).build();
        query
            .execute(&self.pool)
            .await
            .map(|_v| ())
            .map_err(|e| e.into())
    }
}

#[async_trait]
impl InsertRecipePort for RecipeSqliteDS {
    async fn insert_recipe(&self, record: Recipe) -> Result<(), InsertRecipeError> {
        let mut builder =
            QueryBuilder::new("INSERT INTO recipe (uuid, name, image, method ) VALUES (");
        let recipe_insert_query = builder
            .push_bind(record.uuid().to_string())
            .push(", ")
            .push_bind(record.name())
            .push(", ")
            .push_bind(record.image())
            .push(", ")
            .push_bind(record.method())
            .push(") ")
            .build();
        let mut builder = QueryBuilder::new("INSERT INTO ingredient (uuid, name, amount, unit) ");
        let ingredient_insert_query = builder
            .push_values(record.ingredients().iter(), |mut q, item| {
                q.push_bind(item.uuid().to_string())
                    .push_bind(item.name())
                    .push_bind(item.amount())
                    .push_bind(item.unit());
            })
            .build();

        let mut builder =
            QueryBuilder::new("INSERT INTO recipe_ingredient (recipe_uuid, ingredient_uuid ) ");
        let nm_insert_query = builder
            .push_values(record.ingredients().iter(), |mut q, item| {
                q.push_bind(record.uuid().to_string());
                q.push_bind(item.uuid().to_string());
            })
            .build();

        let transaction = self.pool.begin().await?;

        let result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = recipe_insert_query
            .execute(&self.pool)
            .and_then(|_f| ingredient_insert_query.execute(&self.pool))
            .and_then(|_f| nm_insert_query.execute(&self.pool))
            .await;

        if result.is_ok() {
            transaction.commit().await?;
        } else {
            transaction.rollback().await?;
        }
        result.and_then(|_v| Ok(())).map_err(|e| e.into())
    }
}

impl RecipeSqliteDS {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
