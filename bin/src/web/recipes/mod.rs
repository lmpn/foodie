use std::sync::Arc;

use axum::{
    body::Body,
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::configuration::Configuration;
use crate::state::State;
use foodie_core::services::{
    authorization::token_verification_service::TokenVerificationService,
    recipes::{
        create_ingredient_service::CreateIngredient, create_recipe_service::CreateRecipe,
        delete_ingredient_service::DeleteIngredient, delete_recipe_service::DeleteRecipe,
        query_ingredient_page_service::QueryIngredientsPage, query_recipe_service::QueryRecipe,
        update_ingredient_service::UpdateIngredient, update_recipe_service::UpdateRecipe,
    },
};
use foodie_storage::{
    authorization::user_sqlite_ds::UserSqliteDS,
    recipes::{ingredient_sqlite_ds::IngredientSqliteDS, recipes_sqlite_ds::RecipeSqliteDS},
};

use self::{
    create_ingredient_handler::DynCreateIngredientService,
    create_recipe_handler::DynCreateRecipeService,
    delete_ingredient_handler::DynDeleteIngredientService,
    delete_recipe_handler::DynDeleteRecipesService,
    read_ingredient_page_handler::DynIngredientsPageQueryService,
    read_recipe_handler::DynQueryRecipeService,
    update_ingredient_handler::DynUpdateIngredientService,
    update_recipe_handler::DynUpdateRecipeService,
};

use super::middleware::authorization::{authorization_middleware, DynTokenVerificationService};

pub mod create_ingredient_handler;
pub mod create_recipe_handler;
pub mod delete_ingredient_handler;
pub mod delete_recipe_handler;
pub mod read_ingredient_page_handler;
pub mod read_recipe_handler;
pub mod update_ingredient_handler;
pub mod update_recipe_handler;

pub fn router(state: State, configuration: &Configuration) -> Router<(), Body> {
    let recipe_storage = RecipeSqliteDS::new(state.pool());
    let user_storage = UserSqliteDS::new(state.pool());
    let ingredient_storage = IngredientSqliteDS::new(state.pool());
    let token_verification_service = Arc::new(TokenVerificationService::new(
        user_storage.clone(),
        configuration.jwt_secret().to_string(),
    )) as DynTokenVerificationService;

    let delete_recipe_service =
        Arc::new(DeleteRecipe::new(recipe_storage.clone())) as DynDeleteRecipesService;
    let query_recipe_service =
        Arc::new(QueryRecipe::new(recipe_storage.clone())) as DynQueryRecipeService;
    let insert_recipe_service =
        Arc::new(CreateRecipe::new(recipe_storage.clone())) as DynCreateRecipeService;
    let update_recipe_service =
        Arc::new(UpdateRecipe::new(recipe_storage.clone())) as DynUpdateRecipeService;
    let create_ingredient_service =
        Arc::new(CreateIngredient::new(ingredient_storage.clone())) as DynCreateIngredientService;
    let delete_ingredient_service =
        Arc::new(DeleteIngredient::new(ingredient_storage.clone())) as DynDeleteIngredientService;
    let update_ingredient_service =
        Arc::new(UpdateIngredient::new(ingredient_storage.clone())) as DynUpdateIngredientService;
    let query_ingredient_page_service =
        Arc::new(QueryIngredientsPage::new(ingredient_storage.clone()))
            as DynIngredientsPageQueryService;
    let ingredients_routes = Router::new()
        .route(
            "/ingredients",
            get(read_ingredient_page_handler::read_ingredient_page_handler),
        )
        .with_state(query_ingredient_page_service)
        .route(
            "/ingredients/:ingredient_identifier",
            put(update_ingredient_handler::update_ingredient_handler),
        )
        .with_state(update_ingredient_service)
        .route(
            "/ingredients/:ingredient_identifier",
            delete(delete_ingredient_handler::delete_ingredient_handler),
        )
        .with_state(delete_ingredient_service)
        .route(
            "/ingredients",
            post(create_ingredient_handler::create_ingredient_handler),
        )
        .with_state(create_ingredient_service);
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
            get(read_recipe_handler::read_recipe_handler),
        )
        .with_state(query_recipe_service.clone())
        .route("/", post(create_recipe_handler::insert_recipe_handler))
        .with_state(insert_recipe_service.clone());

    let recipes_router = Router::new()
        .nest("/recipes", recipes_routes)
        .nest("/recipes/:identifier", ingredients_routes)
        .route_layer(middleware::from_fn_with_state(
            token_verification_service,
            authorization_middleware,
        ));
    Router::new().nest("/api/v1", recipes_router)
}
