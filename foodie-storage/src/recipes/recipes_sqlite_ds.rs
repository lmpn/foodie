use async_trait::async_trait;
use sqlx::SqlitePool;

use foodie_core::{
    domain::recipe::Recipe,
    ports::outgoing::recipe::{
        delete_recipe_port::{DeleteRecipeError, DeleteRecipePort},
        insert_recipe_port::{InsertRecipeError, InsertRecipePort},
        query_recipe_port::{QueryRecipeError, QueryRecipePort},
        query_recipes_port::{QueryRecipesError, QueryRecipesPort},
        update_recipe_port::{UpdateRecipeError, UpdateRecipePort},
    },
};

#[derive(Debug, sqlx::FromRow, Clone)]
struct RecipeRecord {
    pub uuid: String,
    pub name: String,
    pub image: String,
}

impl From<RecipeRecord> for Recipe {
    fn from(val: RecipeRecord) -> Self {
        Recipe::new(
            uuid::Uuid::parse_str(&val.uuid).unwrap_or_default(),
            val.name,
            val.image,
            None,
        )
    }
}

#[derive(Clone, Debug)]
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
        let e = sqlx::query!(
            r#" UPDATE recipe SET name = ?, method = ?, image = ?  WHERE uuid = ?"#,
            name,
            method,
            image,
            uuid
        )
        .execute(&self.pool)
        .await;
        if e.is_ok() && e.as_ref().unwrap().rows_affected() == 0 {
            return Err(UpdateRecipeError::RecordNotFound);
        } else {
            e.map(|_e| ()).map_err(|e| match e {
                sqlx::Error::RowNotFound => UpdateRecipeError::RecordNotFound,
                _ => UpdateRecipeError::InternalError,
            })
        }
    }
}

#[async_trait]
impl QueryRecipePort for RecipeSqliteDS {
    async fn query_recipe(&self, uuid: uuid::Uuid) -> Result<Recipe, QueryRecipeError> {
        let uuid = uuid.to_string();
        sqlx::query_as!(
            RecipeRecord,
            r#"SELECT uuid, name, image FROM recipe WHERE recipe.uuid = ?"#,
            uuid
        )
        .fetch_one(&self.pool)
        .await
        .map(Into::into)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => QueryRecipeError::RecordNotFound,
            _ => QueryRecipeError::InternalError,
        })
    }
}

#[async_trait]
impl DeleteRecipePort for RecipeSqliteDS {
    async fn delete_recipe(&self, uuid: &str) -> Result<(), DeleteRecipeError> {
        let e = sqlx::query!("DELETE FROM recipe WHERE uuid = ?", uuid)
            .execute(&self.pool)
            .await;
        if e.is_ok() && e.as_ref().unwrap().rows_affected() == 0 {
            return Err(DeleteRecipeError::RecipeNotFound);
        } else {
            e.map(|_e| ()).map_err(|_| DeleteRecipeError::InternalError)
        }
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
        .map_err(|e| match e {
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    return InsertRecipeError::RecipeAlreadyExists;
                }
                InsertRecipeError::InternalError
            }
            _ => InsertRecipeError::InternalError,
        })
    }
}

#[async_trait]
impl QueryRecipesPort for RecipeSqliteDS {
    async fn query_recipes(&self, count: u8, page: u8) -> Result<Vec<Recipe>, QueryRecipesError> {
        let o = page * count;
        sqlx::query_as!(
            RecipeRecord,
            r#"SELECT uuid, name, image FROM recipe LIMIT ? OFFSET ?"#,
            count,
            o
        )
        .fetch_all(&self.pool)
        .await
        .map(|e| e.into_iter().map(Into::into).collect())
        .map_err(|_| QueryRecipesError::InternalError)
    }
}
