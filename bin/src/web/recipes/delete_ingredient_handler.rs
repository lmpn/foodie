use axum::{
    body::{self, BoxBody},
    extract::Path,
    http::{Response, StatusCode},
    Json,
};
use foodie_core::ports::incoming::recipe::delete_ingredient_command::{
    DeleteIngredientCommand, DeleteIngredientCommandError,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::FoodieError;
pub(crate) type DynDeleteIngredientService = Arc<dyn DeleteIngredientCommand + Send + Sync>;

pub async fn delete_ingredient_handler(
    axum::extract::State(service): axum::extract::State<DynDeleteIngredientService>,
    Path((recipe_identifier, ingredient_identifier)): Path<(Uuid, Uuid)>,
) -> Result<Response<BoxBody>, FoodieError> {
    let builder = match service
        .delete(recipe_identifier, ingredient_identifier)
        .await
    {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
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
    };
    builder.map_err(|e| e.into())
}
