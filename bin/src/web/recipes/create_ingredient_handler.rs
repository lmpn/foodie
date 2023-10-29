use std::sync::Arc;

use axum::{
    body::{self},
    extract::Path,
    response::Response,
    Json,
};
use hyper::{Body, StatusCode};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use foodie_core::ports::incoming::recipe::create_ingredient_command::{
    CreateIngredientCommand, CreateIngredientCommandError, Request,
};

use crate::error::FoodieError;
#[derive(Debug, Clone, Deserialize)]
pub struct AddIngredientJson {
    name: String,
    amount: f64,
    unit: String,
}

pub(crate) type DynCreateIngredientService = Arc<dyn CreateIngredientCommand + Sync + Send>;

pub async fn create_ingredient_handler(
    axum::extract::State(service): axum::extract::State<DynCreateIngredientService>,
    Path(recipe_uuid): Path<Uuid>,
    Json(body): Json<AddIngredientJson>,
) -> Result<Response<Body>, FoodieError> {
    let request = Request::new(recipe_uuid, body.name, body.amount, body.unit);
    let result = service.create_ingredient(request).await;
    match result {
        Ok(uuid) => Response::builder()
            .status(StatusCode::CREATED)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(Json(json!({"uuid":uuid})).to_string()))
            .map_err(|e| e.into()),
        Err(CreateIngredientCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{:?}", CreateIngredientCommandError::RecipeNotFound),
                }))
                .to_string(),
            ))
            .map_err(|e| e.into()),
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{:?}", e),
                }))
                .to_string(),
            ))
            .map_err(|e| e.into()),
    }
}
