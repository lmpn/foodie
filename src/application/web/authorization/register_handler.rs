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

use crate::{
    application::domain::authorization::filtered_user::FilteredUser,
    application::ports::incoming::authorization::registration_service::{
        RegistrationService, RegistrationServiceError,
    },
    error::YaissError,
};

#[derive(Debug, Deserialize)]
pub struct UserRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
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

pub(crate) type DynRegistrationService = Arc<dyn RegistrationService + Sync + Send>;

pub async fn register_user_handler(
    State(service): State<DynRegistrationService>,
    Json(body): Json<UserRegistration>,
) -> Result<Response<BoxBody>, YaissError> {
    let builder = match service.register(body.name, body.email, body.password).await {
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
        Err(RegistrationServiceError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "status": "fail",
                    "error": format!("{}", RegistrationServiceError::InternalError)
                }))
                .to_string(),
            )),
        Err(RegistrationServiceError::UserAlreadyExists) => Response::builder()
            .status(StatusCode::CONFLICT)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "status": "fail",
                    "error": format!("{}", RegistrationServiceError::UserAlreadyExists)
                }))
                .to_string(),
            )),
        Err(RegistrationServiceError::PasswordHash) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "status": "fail",
                    "error": format!("{}", RegistrationServiceError::PasswordHash)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
