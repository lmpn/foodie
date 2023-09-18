use crate::{
    application::{
        domain::recipe::{ingredient::Ingredient, recipe::Recipe},
        ports::incoming::recipe::read_partial_query::{ReadPartialError, ReadPartialQuery},
    },
    error::YaissError,
};
use axum::{
    body::{self, Body},
    http::{Response, StatusCode},
    Json,
};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct IngredientJson {
    uuid: uuid::Uuid,
    name: String,
    amount: f64,
    unit: String,
}

impl From<&Ingredient> for IngredientJson {
    fn from(value: &Ingredient) -> Self {
        Self {
            uuid: value.uuid(),
            name: value.name().to_string(),
            amount: value.amount(),
            unit: value.unit().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RecipeJson {
    uuid: uuid::Uuid,
    name: String,
    image: String,
    method: String,
    ingredients: Vec<IngredientJson>,
}

impl From<Recipe> for RecipeJson {
    fn from(value: Recipe) -> Self {
        Self {
            uuid: value.uuid(),
            name: value.name().to_string(),
            image: value.image().to_string(),
            method: value.method().to_string(),
            ingredients: value
                .ingredients()
                .iter()
                .map(|e| IngredientJson::from(e))
                .collect::<Vec<IngredientJson>>(),
        }
    }
}

pub(crate) type DynQueryRecipeService = Arc<dyn ReadPartialQuery + Sync + Send>;
pub async fn query_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynQueryRecipeService>,
    index: axum::extract::Path<uuid::Uuid>,
) -> Result<Response<Body>, YaissError> {
    let builder = match service.clone().read_recipe(index.0).await {
        Ok(recipe) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!(RecipeJson::from(recipe))).to_string(),
            )),
        Err(ReadPartialError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{}", ReadPartialError::RecipeNotFound)
                }))
                .to_string(),
            )),
        Err(ReadPartialError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{}", ReadPartialError::InternalError)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
