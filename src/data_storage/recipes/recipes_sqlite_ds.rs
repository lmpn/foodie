use async_trait::async_trait;
use futures::TryFutureExt;
use sqlx::{Execute, QueryBuilder, Sqlite, SqlitePool};
use tracing::info;
use uuid::Uuid;

use crate::services::recipes::{
    domain::{ingredient, recipe::Recipe},
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
    async fn update_recipe(&self, record: Recipe) -> Result<(), UpdateRecipeError> {
        Err(UpdateRecipeError::InternalError)
    }
}

#[async_trait]
impl QueryRecipePort for RecipeSqliteDS {
    async fn query_recipe(&self, index: uuid::Uuid) -> Result<Recipe, QueryRecipeError> {
        Err(QueryRecipeError::InternalError)
    }
}

#[async_trait]
impl DeleteRecipePort for RecipeSqliteDS {
    async fn delete_recipe(&self, uuid: Uuid) -> Result<(), DeleteRecipeError> {
        let mut builder = QueryBuilder::new("SELECT * FROM recipe WHERE uuid = ");
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
                q.push_bind(record.uuid().to_string())
                    .push_bind(item.uuid().to_string());
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
