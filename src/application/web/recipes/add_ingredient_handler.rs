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

use crate::{
    application::ports::incoming::recipe::add_ingredient_command::{
        AddIngredientCommand, AddIngredientCommandError, Request,
    },
    error::YaissError,
};

#[derive(Debug, Clone, Deserialize)]
pub struct AddIngredientJson {
    name: String,
    amount: f64,
    unit: String,
}

pub(crate) type DynAddIngredientService = Arc<dyn AddIngredientCommand + Sync + Send>;

pub async fn add_ingredient_handler(
    axum::extract::State(service): axum::extract::State<DynAddIngredientService>,
    Path(recipe_uuid): Path<Uuid>,
    Json(body): Json<AddIngredientJson>,
) -> Result<Response<Body>, YaissError> {
    let request = Request::new(recipe_uuid, body.name, body.amount, body.unit);
    let result = service.add_ingredient(request).await;
    match result {
        Ok(uuid) => Response::builder()
            .status(StatusCode::CREATED)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(Json(json!({"uuid":uuid})).to_string()))
            .map_err(|e| e.into()),
        Err(AddIngredientCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{:?}", AddIngredientCommandError::RecipeNotFound),
                }))
                .to_string(),
            ))
            .map_err(|e| e.into()),
        Err(AddIngredientCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{:?}", AddIngredientCommandError::InternalError),
                }))
                .to_string(),
            ))
            .map_err(|e| e.into()),
    }
}
