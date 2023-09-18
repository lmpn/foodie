use crate::{
    application::ports::incoming::recipe::delete_command::{DeleteCommand, DeleteCommandError},
    error::YaissError,
};
use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) type DynDeleteRecipesService = Arc<dyn DeleteCommand + Send + Sync>;

pub async fn delete_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynDeleteRecipesService>,
    identifier: axum::extract::Path<Uuid>,
) -> Result<Response<BoxBody>, YaissError> {
    let builder = match service.delete(identifier.0).await {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(BoxBody::default()),
        Err(DeleteCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteCommandError::InternalError)
                }))
                .to_string(),
            )),
        Err(DeleteCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteCommandError::RecipeNotFound)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
