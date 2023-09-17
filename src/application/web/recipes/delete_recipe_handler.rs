use crate::{
    application::ports::incoming::recipe::delete_recipe_service::{
        DeleteRecipeService, DeleteRecipeServiceError,
    },
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

pub(crate) type DynDeleteRecipesService = Arc<dyn DeleteRecipeService + Send + Sync>;

pub async fn delete_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynDeleteRecipesService>,
    identifier: axum::extract::Path<Uuid>,
) -> Result<Response<BoxBody>, YaissError> {
    let builder = match service.delete_recipe(identifier.0).await {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(BoxBody::default()),
        Err(DeleteRecipeServiceError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteRecipeServiceError::InternalError)
                }))
                .to_string(),
            )),
        Err(DeleteRecipeServiceError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteRecipeServiceError::RecipeNotFound)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
