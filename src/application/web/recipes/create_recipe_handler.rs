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
) -> Result<Response<BoxBody>, YaissError> {
    let result = service.create_recipe(body.into()).await;
    match result {
        Ok(()) => {
            let builder = Response::builder()
                .status(StatusCode::CREATED)
                .body(body::boxed(BoxBody::default()));
            builder.map_err(|e| e.into())
        }
        Err(CreateRecipeCommandError::InternalError) => {
            let builder = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(
                    Json(json!({
                        "error": format!("{:?}", CreateRecipeCommandError::InternalError),
                    }))
                    .to_string(),
                ));
            builder.map_err(|e| e.into())
        }
    }
}
