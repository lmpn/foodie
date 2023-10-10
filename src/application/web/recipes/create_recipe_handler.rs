use crate::{
    application::ports::incoming::recipe::create_recipe_command::{
        CreateRecipeCommand, CreateRecipeCommandError, Request,
    },
    error::YaissError,
};
use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use hyper::Body;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct InsertRecipeJson {
    name: String,
    image: String,
    method: String,
}

impl Into<Request> for InsertRecipeJson {
    fn into(self) -> Request {
        Request::new(self.name, self.image, self.method)
    }
}

pub(crate) type DynCreateRecipeService = Arc<dyn CreateRecipeCommand + Sync + Send>;
pub async fn insert_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynCreateRecipeService>,
    Json(body): Json<InsertRecipeJson>,
) -> Result<Response<Body>, YaissError> {
    let result = service.create_recipe(body.into()).await;
    match result {
        Ok(uuid) => Response::builder()
            .status(StatusCode::CREATED)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(Json(json!({"uuid":uuid})).to_string()))
            .map_err(|e| e.into()),
        Err(CreateRecipeCommandError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{:?}", CreateRecipeCommandError::InternalError),
                }))
                .to_string(),
            ))
            .map_err(|e| e.into()),
    }
}
