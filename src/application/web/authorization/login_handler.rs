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
    application::ports::incoming::authorization::login_command::{
        LoginCommand, LoginCommandError, Request,
    },
    error::YaissError,
};

#[derive(Debug, Deserialize)]
pub struct UserLoginJson {
    pub email: String,
    pub password: String,
}

impl Into<Request> for UserLoginJson {
    fn into(self) -> Request {
        Request::new(self.email, self.password)
    }
}

pub(crate) type DynLoginService = Arc<dyn LoginCommand + Sync + Send>;

pub async fn login_handler(
    State(service): State<DynLoginService>,
    Json(body): Json<UserLoginJson>,
) -> Result<Response<BoxBody>, YaissError> {
    if body.email.is_empty() {
        let v = Json(json!({
            "status": "fail",
            "error": "email is required"
        }))
        .to_string();
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(v))
            .map_err(Into::into);
    }
    if body.password.is_empty() {
        let v = Json(json!({
            "status": "fail",
            "error": "password is required"
        }))
        .to_string();
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(v))
            .map_err(Into::into);
    }
    let token = service.login(body.into()).await;
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
            LoginCommandError::TokenEncodingError
            | LoginCommandError::InternalError
            | LoginCommandError::PasswordHash,
        ) => {
            let body = Json(json!({
                "status": "fail",
                "token": format!("{}", LoginCommandError::InternalError)
            }))
            .to_string();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(body))
                .map_err(|e| e.into())
        }
        Err(LoginCommandError::InvalidCredentials) => {
            let body = Json(json!({
                "status": "fail",
                "token": format!("{}", LoginCommandError::InvalidCredentials),
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
