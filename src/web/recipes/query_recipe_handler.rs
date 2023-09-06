use std::sync::Arc;

use axum::{
    body::{self, Body},
    http::{Response, StatusCode},
    Json,
};
use serde::Serialize;
use serde_json::json;

use crate::{
    error::YaissError,
    services::recipes::{
        domain::recipe::Recipe, ports::incoming::query_recipe_service::QueryRecipeService,
    },
};

#[derive(Debug, Clone, Serialize)]
pub struct RecipeJson {
    id: i64,
}

impl From<Recipe> for RecipeJson {
    fn from(value: Recipe) -> Self {
        Self { id: value.id() }
    }
}

pub(crate) type DynQueryRecipeService = Arc<dyn QueryRecipeService + Sync + Send>;
pub async fn query_recipe_handler(
    axum::extract::State(_service): axum::extract::State<DynQueryRecipeService>,
    _identifier: axum::extract::Path<i64>,
) -> Result<Response<Body>, YaissError> {
    let builder = Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(body::Body::from(
            Json(json!({
                "error": "not implemented",
            }))
            .to_string(),
        ));
    builder.map_err(|e| e.into())
}
