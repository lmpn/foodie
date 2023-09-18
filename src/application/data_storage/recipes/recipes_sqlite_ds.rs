use async_trait::async_trait;
use futures::TryFutureExt;
use sqlx::{QueryBuilder, SqlitePool};
use tracing::info;
use uuid::Uuid;

use crate::application::{
    domain::recipe::{ingredient::Ingredient, recipe::Recipe},
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

#[derive(Clone)]
pub struct RecipeSqliteDS {
    pool: SqlitePool,
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
        let transaction = self.pool.begin().await?;
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
                row.ruuid
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
