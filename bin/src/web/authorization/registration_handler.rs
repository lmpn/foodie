use std::sync::Arc;

use axum::{
    body::{self, BoxBody},
    extract::State,
    response::Response,
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::FoodieError;
use foodie_core::{
    domain::authorization::filtered_user::FilteredUser,
    ports::incoming::authorization::registration_command::{
        RegistrationCommand, RegistrationCommandError, Request,
    },
};

#[derive(Debug, Deserialize)]
pub struct UserRegistrationJson {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Into<Request> for UserRegistrationJson {
    fn into(self) -> Request {
        Request::new(self.name, self.email, self.password)
    }
}

#[derive(Debug, Serialize)]
pub struct FilteredUserJson {
    id: String,
    name: String,
    email: String,
    role: String,
    photo: String,
    verified: bool,
}

impl From<FilteredUser> for FilteredUserJson {
    fn from(value: FilteredUser) -> Self {
        Self {
            id: value.id().to_string(),
            name: value.name().to_string(),
            email: value.email().to_string(),
            role: value.role().to_string(),
            photo: value.photo().to_string(),
            verified: value.verified(),
        }
    }
}

pub(crate) type DynRegistrationService = Arc<dyn RegistrationCommand + Sync + Send>;

pub async fn registration_handler(
    State(service): State<DynRegistrationService>,
    Json(body): Json<UserRegistrationJson>,
) -> Result<Response<BoxBody>, FoodieError> {
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

    let builder = match service.register(body.into()).await {
        Ok(user) => {
            let v = Json(json!({
                "status": "success",
                "data": serde_json::json!({"user": FilteredUserJson::from(user)})
            }))
            .to_string();
            Response::builder()
                .status(StatusCode::OK)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(v))
        }
        Err(RegistrationCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "status": "fail",
                    "error": format!("{}", RegistrationCommandError::InternalError)
                }))
                .to_string(),
            )),
        Err(RegistrationCommandError::UserAlreadyExists) => Response::builder()
            .status(StatusCode::CONFLICT)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "status": "fail",
                    "error": format!("{}", RegistrationCommandError::UserAlreadyExists)
                }))
                .to_string(),
            )),
        Err(RegistrationCommandError::PasswordHash) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "status": "fail",
                    "error": format!("{}", RegistrationCommandError::PasswordHash)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
