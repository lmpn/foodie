use std::sync::Arc;

use axum::{
    body::{self, BoxBody},
    extract::State,
    response::Response,
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::{
    application::ports::incoming::authorization::login_service::{LoginService, LoginServiceError},
    error::YaissError,
};

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

pub(crate) type DynLoginService = Arc<dyn LoginService + Sync + Send>;

pub async fn login_user_handler(
    State(service): State<DynLoginService>,
    Json(body): Json<UserLogin>,
) -> Result<Response<BoxBody>, YaissError> {
    let token = service.login(body.email, body.password).await;
    match token {
        Ok((token, maxage)) => {
            let cookie = Cookie::build("token", token.to_owned())
                .path("/")
                .max_age(time::Duration::seconds(maxage))
                .same_site(SameSite::Lax)
                .http_only(true)
                .finish();
            let body = Json(json!({
                "status": "success",
                "token": token
            }))
            .to_string();
            Response::builder()
                .status(StatusCode::OK)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .header(axum::http::header::SET_COOKIE, cookie.to_string())
                .body(body::boxed(body))
                .map_err(|e| e.into())
        }
        Err(
            LoginServiceError::TokenEncodingError
            | LoginServiceError::InternalError
            | LoginServiceError::PasswordHash,
        ) => {
            let body = Json(json!({
                "status": "fail",
                "token": format!("{}", LoginServiceError::InternalError)
            }))
            .to_string();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(body))
                .map_err(|e| e.into())
        }
        Err(LoginServiceError::InvalidCredentials) => {
            let body = Json(json!({
                "status": "fail",
                "token": format!("{}", LoginServiceError::InvalidCredentials),
            }))
            .to_string();
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(body))
                .map_err(|e| e.into())
        }
    }
}
