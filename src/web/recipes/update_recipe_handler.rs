use std::sync::Arc;

use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use serde_json::json;

use crate::{
    error::YaissError,
    services::recipes::ports::incoming::update_recipe_service::UpdateRecipeService,
};

pub(crate) type DynUpdateRecipeService = Arc<dyn UpdateRecipeService + Sync + Send>;
pub async fn update_recipe_handler(
    axum::extract::State(_service): axum::extract::State<DynUpdateRecipeService>,
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
