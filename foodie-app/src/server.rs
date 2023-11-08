use crate::api::authorization_api::AuthenticatedUser;
use crate::api::recipe_api::GetRecipes;
use crate::fallback::file_and_error_handler;
use crate::state::AppState;
use crate::{api::authorization_api::AuthorizationSession, landing::Landing};
use axum::{
    body::Body as AxumBody,
    extract::{Path, RawQuery, State},
    http::{header::HeaderMap, Request},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer, SessionSqlitePool};
use foodie_core::services::authorization::service::AuthorizationService;
use foodie_core::services::recipes::service::RecipeService;
use foodie_storage::authorization::user_sqlite_ds::UserSqliteDS;
use foodie_storage::recipes::recipes_sqlite_ds::RecipeSqliteDS;
use leptos::ServerFn;
use leptos::{get_configuration, logging::log, provide_context, use_context, ServerFnError};
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use sqlx::sqlite::SqlitePoolOptions;

pub(crate) fn context_recipe_service() -> Result<RecipeService<RecipeSqliteDS>, ServerFnError> {
    use_context::<RecipeService<RecipeSqliteDS>>()
        .ok_or_else(|| ServerFnError::ServerError("RecipeService missing.".into()))
}

pub(crate) fn context_authorization_service(
) -> Result<AuthorizationService<UserSqliteDS>, ServerFnError> {
    use_context::<AuthorizationService<UserSqliteDS>>()
        .ok_or_else(|| ServerFnError::ServerError("AuthorizationService missing.".into()))
}

pub(crate) fn context_authorization_session_service() -> Result<AuthorizationSession, ServerFnError>
{
    use_context::<AuthorizationSession>()
        .ok_or_else(|| ServerFnError::ServerError("Authorization session missing.".into()))
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    auth_session: AuthorizationSession,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log!("{:?}", path);

    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.authorization_service.clone());
            provide_context(app_state.recipe_service.clone());
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(
    auth_session: AuthorizationSession,
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.recipe_service.clone());
        },
        Landing,
    );
    handler(req).await.into_response()
}

pub async fn server_main() {
    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    let pool = SqlitePoolOptions::new()
        .connect("sqlite:foodie-app/foodie.db")
        .await
        .expect("Could not make pool.");

    // Auth section
    let session_config = SessionConfig::default().with_table_name("axum_sessions");
    let auth_config = AuthConfig::<String>::default();
    let session_store =
        SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config)
            .await
            .expect("Could not make session store");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("could not run SQLx migrations");
    let user_ds = UserSqliteDS::new(pool.clone());
    let recipe_ds = RecipeSqliteDS::new(pool.clone());
    let authorization_service =
        AuthorizationService::new(user_ds.clone(), "secret_jwt".to_string(), 6000);
    let recipe_service = RecipeService::new(recipe_ds.clone());

    GetRecipes::register_explicit().expect("GetRecipes not registered with success");

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(Landing);

    let app_state = AppState {
        leptos_options,
        routes: routes.clone(),
        authorization_service,
        recipe_service,
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(
            AuthSessionLayer::<AuthenticatedUser, String, SessionSqlitePool, UserSqliteDS>::new(
                Some(user_ds),
            )
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
