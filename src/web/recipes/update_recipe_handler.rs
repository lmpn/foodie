use std::sync::Arc;

use axum::{
    body::{self, BoxBody},
    http::{Response, StatusCode},
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::YaissError,
    services::{
        domain::{ingredient::Ingredient, recipe::Recipe},
        ports::incoming::update_recipe_service::{UpdateRecipeService, UpdateRecipeServiceError},
    },
};

#[derive(Debug, Clone, Deserialize)]
pub struct IngredientJson {
    uuid: Option<uuid::Uuid>,
    name: String,
    amount: f64,
    unit: String,
}

impl Into<Ingredient> for IngredientJson {
    fn into(self) -> Ingredient {
        Ingredient::new(
            self.uuid.unwrap_or(uuid::Uuid::default()),
            self.name,
            self.amount,
            self.unit,
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecipeJson {
    uuid: uuid::Uuid,
    name: String,
    image: String,
    method: String,
    update_ingredients: Vec<IngredientJson>,
    delete_ingredients: Vec<uuid::Uuid>,
}

impl Into<Recipe> for RecipeJson {
    fn into(self) -> Recipe {
        Recipe::new(
            self.uuid,
            self.name,
            self.image,
            self.method,
            self.update_ingredients
                .into_iter()
                .map(|e| e.into())
                .collect(),
        )
    }
}

pub(crate) type DynUpdateRecipeService = Arc<dyn UpdateRecipeService + Sync + Send>;
pub async fn update_recipe_handler(
    axum::extract::State(service): axum::extract::State<DynUpdateRecipeService>,
    json: Json<RecipeJson>,
) -> Result<Response<BoxBody>, YaissError> {
    let (delete_ingredients, recipe): (Vec<uuid::Uuid>, Recipe) =
        (json.0.delete_ingredients.clone(), json.0.into());
    let result = service.update_recipe(recipe, delete_ingredients).await;
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
