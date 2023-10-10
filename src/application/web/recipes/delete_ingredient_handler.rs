use crate::{
    application::ports::incoming::recipe::delete_ingredient_command::{
        DeleteIngredientCommand, DeleteIngredientCommandError,
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

pub(crate) type DynDeleteIngredientService = Arc<dyn DeleteIngredientCommand + Send + Sync>;

pub async fn delete_ingredient_handler(
    axum::extract::State(service): axum::extract::State<DynDeleteIngredientService>,
    recipe_identifier: axum::extract::Path<Uuid>,
    ingredient_identifier: axum::extract::Path<Uuid>,
) -> Result<Response<BoxBody>, YaissError> {
    let builder = match service
        .delete(recipe_identifier.0, ingredient_identifier.0)
        .await
    {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(BoxBody::default()),
        Err(DeleteIngredientCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteIngredientCommandError::InternalError)
                }))
                .to_string(),
            )),
        Err(DeleteIngredientCommandError::IngredientNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteIngredientCommandError::IngredientNotFound)
                }))
                .to_string(),
            )),
        Err(DeleteIngredientCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}", DeleteIngredientCommandError::RecipeNotFound)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
