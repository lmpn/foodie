use std::sync::Arc;

use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use serde::Serialize;
use serde_json::json;

use crate::{
    error::YaissError,
    services::recipes::ports::incoming::insert_recipe_service::InsertRecipeService,
};

#[derive(Debug, Clone, Serialize)]
pub struct RecipeJson {
    id: i64,
    updated_on: String,
}

pub(crate) type DynInsertRecipeService = Arc<dyn InsertRecipeService + Sync + Send>;
pub async fn insert_recipe_handler(
    axum::extract::State(_service): axum::extract::State<DynInsertRecipeService>,
) -> Result<Response<BoxBody>, YaissError> {
    let builder = Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(body::boxed(
            Json(json!({
                "error": "not implemented",
            }))
            .to_string(),
        ));
    builder.map_err(|e| e.into())
}
