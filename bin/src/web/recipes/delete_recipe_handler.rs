use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use foodie_core::ports::incoming::recipe::delete_recipe_command::{
    DeleteRecipeCommand, DeleteRecipeCommandError,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::FoodieError;
pub(crate) type DynDeleteRecipesService = Arc<dyn DeleteRecipeCommand + Send + Sync>;

pub async fn delete_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynDeleteRecipesService>,
    identifier: axum::extract::Path<Uuid>,
) -> Result<Response<BoxBody>, FoodieError> {
    let builder = match service.delete_recipe(identifier.0).await {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .body(BoxBody::default()),
        Err(DeleteRecipeCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteRecipeCommandError::InternalError)
                }))
                .to_string(),
            )),
        Err(DeleteRecipeCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteRecipeCommandError::RecipeNotFound)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
