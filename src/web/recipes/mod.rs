use std::sync::Arc;

use axum::{
    body::Body,
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    data_storage::recipes::recipes_sqlite_ds::RecipeSqliteDS,
    services::recipes::{
        delete_recipe_service::DeleteRecipe, insert_recipe_service::InsertRecipe,
        query_recipe_service::QueryRecipe, update_recipe_service::UpdateRecipe,
    },
    state::State,
};

use self::{
    delete_recipe_handler::DynDeleteRecipesService, insert_recipe_handler::DynInsertRecipeService,
    query_recipe_handler::DynQueryRecipeService, update_recipe_handler::DynUpdateRecipeService,
};

pub mod delete_recipe_handler;
pub mod insert_recipe_handler;
pub mod query_recipe_handler;
pub mod update_recipe_handler;

pub fn router(state: State) -> Router<(), Body> {
    let storage = RecipeSqliteDS::new(state.pool());

    let delete_recipe_service =
        Arc::new(DeleteRecipe::new(storage.clone())) as DynDeleteRecipesService;
    let query_recipe_service = Arc::new(QueryRecipe::new(storage.clone())) as DynQueryRecipeService;
    let insert_recipe_service =
        Arc::new(InsertRecipe::new(storage.clone())) as DynInsertRecipeService;
    let update_recipe_service =
        Arc::new(UpdateRecipe::new(storage.clone())) as DynUpdateRecipeService;

    let recipes_routes = Router::new()
        .route(
            "/:identifier",
            put(update_recipe_handler::update_recipe_handler),
        )
        .with_state(update_recipe_service)
        .route(
            "/:identifier",
            delete(delete_recipe_handler::delete_recipe_handler),
        )
        .with_state(delete_recipe_service)
        .route(
            "/:identifier",
            get(query_recipe_handler::query_recipe_handler),
        )
        .with_state(query_recipe_service.clone())
        .route("/", post(insert_recipe_handler::insert_recipe_handler))
        .with_state(insert_recipe_service.clone());

    let recipes_router = Router::new().nest("/recipes", recipes_routes);
    Router::new().nest("/api/v1", recipes_router)
}
