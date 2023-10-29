use crate::error::FoodieError;
use axum::{
    body::{self, BoxBody},
    extract::Path,
    http::{Response, StatusCode},
    Json,
};
use foodie_core::ports::incoming::recipe::update_ingredient_command::{
    Request, UpdateIngredientCommand, UpdateIngredientCommandError,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateIngredientJson {
    name: String,
    amount: f64,
    unit: String,
}

pub(crate) type DynUpdateIngredientService = Arc<dyn UpdateIngredientCommand + Sync + Send>;
pub async fn update_ingredient_handler(
    axum::extract::State(service): axum::extract::State<DynUpdateIngredientService>,
    Path((recipe_identifier, ingredient_identifier)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateIngredientJson>,
) -> Result<Response<BoxBody>, FoodieError> {
    let request = Request::new(
        ingredient_identifier,
        body.name,
        body.amount,
        body.unit,
        recipe_identifier,
    );
    let result = service.update_ingredient(request).await;
    let builder = match result {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .body(BoxBody::default()),
        Err(UpdateIngredientCommandError::IngredientNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateIngredientCommandError::IngredientNotFound)
                }))
                .to_string(),
            )),
        Err(UpdateIngredientCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{}",UpdateIngredientCommandError::RecipeNotFound)
                }))
                .to_string(),
            )),
        Err(UpdateIngredientCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateIngredientCommandError::InternalError)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
