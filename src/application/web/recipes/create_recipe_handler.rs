use crate::{
    application::ports::incoming::recipe::create_command::{
        CreateCommand, CreateCommandError, Request,
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

pub(crate) type DynCreateRecipeService = Arc<dyn CreateCommand + Sync + Send>;
pub async fn insert_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynCreateRecipeService>,
    Json(body): Json<InsertRecipeJson>,
) -> Result<Response<BoxBody>, YaissError> {
    let result = service.insert(body.into()).await;
    match result {
        Ok(()) => {
            let builder = Response::builder()
                .status(StatusCode::CREATED)
                .body(body::boxed(BoxBody::default()));
            builder.map_err(|e| e.into())
        }
        Err(CreateCommandError::InternalError) => {
            let builder = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(body::boxed(
                    Json(json!({
                        "error": format!("{:?}", CreateCommandError::InternalError),
                    }))
                    .to_string(),
                ));
            builder.map_err(|e| e.into())
        }
    }
}
