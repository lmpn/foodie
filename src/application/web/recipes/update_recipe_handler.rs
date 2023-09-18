use crate::{
    application::ports::incoming::recipe::update_recipe_service::{
        Request, UpdateRecipeService, UpdateRecipeServiceError,
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

pub(crate) type DynUpdateRecipeService = Arc<dyn UpdateRecipeService + Sync + Send>;
pub async fn update_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynUpdateRecipeService>,
    json: Json<UpdateRecipeJson>,
) -> Result<Response<BoxBody>, YaissError> {
    let result = service.update_recipe(json.0.into()).await;
    let builder = match result {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(BoxBody::default()),
        Err(UpdateRecipeServiceError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateRecipeServiceError::RecipeNotFound)
                }))
                .to_string(),
            )),
        Err(UpdateRecipeServiceError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateRecipeServiceError::InternalError)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
