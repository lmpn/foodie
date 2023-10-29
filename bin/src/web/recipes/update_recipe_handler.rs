use crate::error::FoodieError;
use axum::{
    body::{self},
    http::{Response, StatusCode},
    Json,
};
use foodie_core::ports::incoming::recipe::update_recipe_command::{
    Request, UpdateRecipeCommand, UpdateRecipeCommandError,
};
use hyper::Body;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tracing::info;
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRecipeJson {
    name: String,
    image: String,
    method: String,
}

pub(crate) type DynUpdateRecipeService = Arc<dyn UpdateRecipeCommand + Sync + Send>;
pub async fn update_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynUpdateRecipeService>,
    axum::extract::Path(recipe_uuid): axum::extract::Path<uuid::Uuid>,
    Json(body): Json<UpdateRecipeJson>,
) -> Result<Response<Body>, FoodieError> {
    info!("{:?}", body);
    if body.name.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": "name is required"
                }))
                .to_string(),
            ))
            .map_err(|e| e.into());
    }
    let request = Request::new(recipe_uuid, body.name, body.image, body.method);
    let result = service.update_recipe(request).await;
    let builder = match result {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .body(body::Body::empty()),
        Err(UpdateRecipeCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{}",UpdateRecipeCommandError::RecipeNotFound)
                }))
                .to_string(),
            )),
        Err(UpdateRecipeCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{}",UpdateRecipeCommandError::InternalError)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
