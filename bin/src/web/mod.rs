use axum::{body, response::Response, Json};
use hyper::{Body, StatusCode};
use serde_json::json;

use crate::error::FoodieError;

pub mod authorization;
pub mod middleware;
pub mod recipes;
pub async fn handler_404() -> Result<Response<Body>, FoodieError> {
    let body = Json(json!({
        "error": "resource not found",
    }))
    .to_string();
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(body::Body::from(body))
        .map_err(|e| e.into())
}