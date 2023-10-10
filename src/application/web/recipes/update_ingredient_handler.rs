use crate::{
    application::ports::incoming::recipe::update_ingredient_command::{
        Request, UpdateIngredientCommand, UpdateIngredientCommandError,
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
pub struct UpdateIngredientJson {
    uuid: uuid::Uuid,
    name: String,
    amount: f64,
    unit: String,
    recipe_uuid: uuid::Uuid,
}

impl Into<Request> for UpdateIngredientJson {
    fn into(self) -> Request {
        Request::new(
            self.uuid,
            self.name,
            self.amount,
            self.unit,
            self.recipe_uuid,
        )
    }
}

pub(crate) type DynUpdateIngredientService = Arc<dyn UpdateIngredientCommand + Sync + Send>;
pub async fn update_ingredient_handler(
    axum::extract::State(service): axum::extract::State<DynUpdateIngredientService>,
    Json(body): Json<UpdateIngredientJson>,
) -> Result<Response<BoxBody>, YaissError> {
    let result = service.update_ingredient(body.into()).await;
    let builder = match result {
        Ok(()) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(BoxBody::default()),
        Err(UpdateIngredientCommandError::IngredientNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateIngredientCommandError::IngredientNotFound)
                }))
                .to_string(),
            )),
        Err(UpdateIngredientCommandError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateIngredientCommandError::RecipeNotFound)
                }))
                .to_string(),
            )),
        Err(UpdateIngredientCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::boxed(
                Json(json!({
                    "error": format!("{:?}",UpdateIngredientCommandError::InternalError)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
