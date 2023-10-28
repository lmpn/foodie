use crate::error::FoodieError;
use axum::{
    body::{self, BoxBody},
    response::Response,
    Json,
};
use axum_extra::extract::cookie::Cookie;
use hyper::StatusCode;
use serde_json::json;

pub async fn logout_handler() -> Result<Response<BoxBody>, FoodieError> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .http_only(true)
        .finish();

    let body = Json(json!({
        "status": "success"
    }))
    .to_string();
    Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .header(axum::http::header::SET_COOKIE, cookie.to_string())
        .body(body::boxed(body))
        .map_err(|e| e.into())
}
