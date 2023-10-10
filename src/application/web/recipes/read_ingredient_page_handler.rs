use axum::{
    body::{self, Body},
    http::{Response, StatusCode},
    Json,
};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

use crate::{
    application::{
        domain::recipe::ingredient::Ingredient,
        ports::incoming::recipe::ingredient_page_query::{
            IngredientsPageQuery, IngredientsPageQueryError,
        },
    },
    error::YaissError,
};

#[derive(Serialize)]
struct IngredientJson {
    uuid: uuid::Uuid,
    name: String,
    amount: f64,
    unit: String,
}

#[derive(Serialize)]
struct IngredientsListJson {
    pub ingredients: Vec<IngredientJson>,
}

impl From<Vec<Ingredient>> for IngredientsListJson {
    fn from(value: Vec<Ingredient>) -> Self {
        let ingredient_json = value.iter().map(From::from).collect();
        Self {
            ingredients: ingredient_json,
        }
    }
}

impl From<&Ingredient> for IngredientJson {
    fn from(value: &Ingredient) -> Self {
        IngredientJson {
            uuid: value.uuid(),
            name: value.name().to_string(),
            amount: value.amount(),
            unit: value.unit().to_string(),
        }
    }
}

pub(crate) type DynIngredientsPageQueryService = Arc<dyn IngredientsPageQuery + Sync + Send>;
pub async fn read_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynIngredientsPageQueryService>,
    axum::extract::Path(uuid): axum::extract::Path<uuid::Uuid>,
    axum::extract::Query(count): axum::extract::Query<i64>,
    axum::extract::Query(offset): axum::extract::Query<i64>,
) -> Result<Response<Body>, YaissError> {
    let builder = match service
        .clone()
        .ingredients_page_query(uuid, count, offset)
        .await
    {
        Ok(ingredients) => Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!(IngredientsListJson::from(ingredients))).to_string(),
            )),
        Err(IngredientsPageQueryError::RecipeNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{}", IngredientsPageQueryError::RecipeNotFound)
                }))
                .to_string(),
            )),
        Err(IngredientsPageQueryError::InternalError) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(body::Body::from(
                Json(json!({
                    "error": format!("{}", IngredientsPageQueryError::InternalError)
                }))
                .to_string(),
            )),
    };
    builder.map_err(|e| e.into())
}
