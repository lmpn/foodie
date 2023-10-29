use axum::{
    body::{self, Body},
    extract::Query,
    http::{Response, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::error::FoodieError;
use foodie_core::{
    domain::recipe::ingredient::Ingredient,
    ports::incoming::recipe::ingredient_page_query::{
        IngredientsPageQuery, IngredientsPageQueryError,
    },
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

#[derive(Deserialize)]
pub struct Pagination {
    pub(crate) count: i64,
    pub(crate) offset: i64,
}

pub(crate) type DynIngredientsPageQueryService = Arc<dyn IngredientsPageQuery + Sync + Send>;
pub async fn read_ingredient_page_handler(
    axum::extract::State(service): axum::extract::State<DynIngredientsPageQueryService>,
    axum::extract::Path(uuid): axum::extract::Path<uuid::Uuid>,
    pagination: Query<Pagination>,
) -> Result<Response<Body>, FoodieError> {
    let builder = match service
        .clone()
        .ingredients_page_query(uuid, pagination.count, pagination.offset)
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
