use axum::{
    body::{self, Body},
    http::{Response, StatusCode},
    Json,
};
use foodie_backend::application::{
    domain::recipe::recipe::Recipe,
    ports::incoming::recipe::recipe_query::{RecipeQuery, RecipeQueryError},
};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

use crate::error::FoodieError;
#[derive(Debug, Clone, Serialize)]
pub struct RecipeJson {
    uuid: uuid::Uuid,
    name: String,
    image: String,
    method: String,
}

impl From<Recipe> for RecipeJson {
    fn from(value: Recipe) -> Self {
        Self {
            uuid: value.uuid(),
            name: value.name().to_string(),
            image: value.image().to_string(),
            method: value.method().to_string(),
        }
    }
}

pub(crate) type DynQueryRecipeService = Arc<dyn RecipeQuery + Sync + Send>;
pub async fn read_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynQueryRecipeService>,
    index: axum::extract::Path<uuid::Uuid>,
) -> Result<Response<Body>, FoodieError> {
    let builder = match service.clone().recipe_query(index.0).await {
        Ok(recipe) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!(RecipeJson::from(recipe))).to_string(),
            )),
        Err(RecipeQueryError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{}", RecipeQueryError::RecipeNotFound)
                }))
                .to_string(),
            )),
        Err(RecipeQueryError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{}", RecipeQueryError::InternalError)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
