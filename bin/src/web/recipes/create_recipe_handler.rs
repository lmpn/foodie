use crate::error::FoodieError;
use axum::{
    body::{self},
    http::{Response, StatusCode},
    Json,
};
use foodie_core::ports::incoming::recipe::create_recipe_command::{CreateRecipeCommand, Request};
use hyper::Body;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct InsertRecipeJson {
    name: String,
    image: String,
    method: String,
}

impl From<InsertRecipeJson> for Request {
    fn from(val: InsertRecipeJson) -> Self {
        Request::new(val.name, val.image, val.method)
    }
}

pub(crate) type DynCreateRecipeService = Arc<dyn CreateRecipeCommand + Sync + Send>;
pub async fn insert_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynCreateRecipeService>,
    Json(body): Json<InsertRecipeJson>,
) -> Result<Response<Body>, FoodieError> {
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
    let result = service.create_recipe(body.into()).await;
    match result {
        Ok(uuid) => Response::builder()
            .status(StatusCode::CREATED)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(Json(json!({"uuid":uuid})).to_string()))
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
