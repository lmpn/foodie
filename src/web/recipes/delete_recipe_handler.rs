use std::sync::Arc;

use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use serde_json::json;

use crate::{
    error::YaissError,
    services::recipes::ports::incoming::delete_recipe_service::DeleteRecipeService,
};

pub(crate) type DynDeleteRecipesService = Arc<dyn DeleteRecipeService + Send + Sync>;

pub async fn delete_recipe_handler(
    axum::extract::State(_service): axum::extract::State<DynDeleteRecipesService>,
    _identifier: axum::extract::Path<i64>,
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
