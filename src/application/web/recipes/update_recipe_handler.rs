use crate::{
    application::ports::incoming::recipe::update_command::{
        Request, UpdateCommand, UpdateCommandError,
    },
    error::YaissError,
};
use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRecipeJson {
    uuid: uuid::Uuid,
    name: String,
    image: String,
    method: String,
}

impl Into<Request> for UpdateRecipeJson {
    fn into(self) -> Request {
        Request::new(self.uuid, self.name, self.image, self.method)
    }
}

pub(crate) type DynUpdateRecipeService = Arc<dyn UpdateCommand + Sync + Send>;
pub async fn update_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynUpdateRecipeService>,
    Json(body): Json<UpdateRecipeJson>,
) -> Result<Response<BoxBody>, YaissError> {
    let result = service.update(body.into()).await;
    let builder = match result {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(BoxBody::default()),
        Err(UpdateCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateCommandError::RecipeNotFound)
                }))
                .to_string(),
            )),
        Err(UpdateCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateCommandError::InternalError)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
