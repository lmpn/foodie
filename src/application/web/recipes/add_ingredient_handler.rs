use std::sync::Arc;

use axum::{
    body::{self, BoxBody},
    response::Response,
    Json,
};
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    application::ports::incoming::recipe::add_ingredient_command::{
        AddIngredientCommand, AddIngredientCommandError, Request,
    },
    error::YaissError,
};

#[derive(Debug, Clone, Deserialize)]
pub struct AddIngredientJson {
    recipe_id: Uuid,
    name: String,
    amount: f64,
    unit: String,
}

impl Into<Request> for AddIngredientJson {
    fn into(self) -> Request {
        Request::new(self.recipe_id, self.name, self.amount, self.unit)
    }
}

pub(crate) type DynAddIngredientService = Arc<dyn AddIngredientCommand + Sync + Send>;

pub async fn add_ingredient_handler(
    axum::extract::State(service): axum::extract::State<DynAddIngredientService>,
    Json(body): Json<AddIngredientJson>,
) -> Result<Response<BoxBody>, YaissError> {
    let result = service.add(body.into()).await;
    match result {
        Ok(()) => {
            let builder = Response::builder()
                .status(StatusCode::CREATED)
                .body(body::boxed(BoxBody::default()));
            builder.map_err(|e| e.into())
        }
        Err(AddIngredientCommandError::RecipeNotFound) => {
            let builder = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(
                    Json(json!({
                        "error": format!("{:?}", AddIngredientCommandError::RecipeNotFound),
                    }))
                    .to_string(),
                ));
            builder.map_err(|e| e.into())
        }
        Err(AddIngredientCommandError::InternalError) => {
            let builder = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(
                    Json(json!({
                        "error": format!("{:?}", AddIngredientCommandError::InternalError),
                    }))
                    .to_string(),
                ));
            builder.map_err(|e| e.into())
        }
    }
}
