use axum::extract::FromRef;
use foodie_core::services::authorization::service::AuthorizationService;
use foodie_core::services::recipes::service::RecipeService;
use foodie_storage::authorization::user_sqlite_ds::UserSqliteDS;
use foodie_storage::recipes::recipes_sqlite_ds::RecipeSqliteDS;
use leptos::LeptosOptions;
use leptos_router::RouteListing;
/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<RouteListing>,
    pub authorization_service: AuthorizationService<UserSqliteDS>,
    pub recipe_service: RecipeService<RecipeSqliteDS>,
}
