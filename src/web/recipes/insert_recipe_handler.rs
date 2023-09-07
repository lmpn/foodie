use std::sync::Arc;

use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::YaissError,
    services::recipes::{
        domain::{ingredient::Ingredient, recipe::Recipe},
        ports::incoming::insert_recipe_service::{InsertRecipeService, InsertRecipeServiceError},
    },
};

#[derive(Debug, Clone, Deserialize)]
pub struct IngredientJson {
    name: String,
    amount: f64,
    unit: String,
}

impl Into<Ingredient> for IngredientJson {
    fn into(self) -> Ingredient {
        Ingredient::new(uuid::Uuid::new_v4(), self.name, self.amount, self.unit)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecipeJson {
    name: String,
    image: String,
    method: String,
    ingredients: Vec<IngredientJson>,
}

impl Into<Recipe> for RecipeJson {
    fn into(self) -> Recipe {
        Recipe::new(
            uuid::Uuid::new_v4(),
            self.name,
            self.image,
            self.method,
            self.ingredients
                .into_iter()
                .map(IngredientJson::into)
                .collect::<Vec<Ingredient>>(),
        )
    }
}

pub(crate) type DynInsertRecipeService = Arc<dyn InsertRecipeService + Sync + Send>;
pub async fn insert_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynInsertRecipeService>,
    recipe: Json<RecipeJson>,
) -> Result<Response<BoxBody>, YaissError> {
    let result = service.insert_recipe(recipe.0.into()).await;
    match result {
        Ok(()) => {
            let builder = Response::builder()
                .status(StatusCode::CREATED)
                .body(body::boxed(BoxBody::default()));
            builder.map_err(|e| e.into())
        }
        Err(InsertRecipeServiceError::NoIngredients) => {
            let builder = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(
                    Json(json!({
                        "error": format!("{:?}", InsertRecipeServiceError::NoIngredients),
                    }))
                    .to_string(),
                ));
            builder.map_err(|e| e.into())
        }
        Err(InsertRecipeServiceError::InternalError) => {
            let builder = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(
                    Json(json!({
                        "error": format!("{:?}", InsertRecipeServiceError::InternalError),
                    }))
                    .to_string(),
                ));
            builder.map_err(|e| e.into())
        }
    }
}
