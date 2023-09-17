use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use hyper::Body;

use crate::{
    configuration::{self, Configuration},
    data_storage::authorization::user_sqlite_ds::UserSqliteDS,
    services::authorization::{
        login_service::Login, registration_service::Registration,
        token_verification_service::TokenVerification,
    },
    state::State,
};

use self::{
    login_handler::DynLoginService, logout_handler::logout_user_handler,
    register_handler::DynRegistrationService,
};

use super::middleware::authorization::{authrorization_middleware, DynTokenVerificationService};

pub mod login_handler;
pub mod logout_handler;
pub mod register_handler;
pub fn router(state: State, configuration: &Configuration) -> Router<(), Body> {
    let storage = UserSqliteDS::new(state.pool());

    let token_verification_service = Arc::new(TokenVerification::new(
        storage.clone(),
        configuration.jwt_secret().to_string(),
    )) as DynTokenVerificationService;
    let login_service = Arc::new(Login::new(
        storage.clone(),
        configuration.jwt_secret().to_string(),
        configuration.jwt_maxage(),
    )) as DynLoginService;
    let registration_service =
        Arc::new(Registration::new(storage.clone())) as DynRegistrationService;

    let authorization_routes = Router::new()
        .route("/logout", get(logout_user_handler))
        .route_layer(middleware::from_fn_with_state(
            token_verification_service,
            authrorization_middleware,
        ))
        .route("/register", post(register_handler::register_user_handler))
        .with_state(registration_service)
        .route("/login", post(login_handler::login_user_handler))
        .with_state(login_service);

    let authorization_routes = Router::new().nest("/authorization", authorization_routes);
    Router::new().nest("/api/v1", authorization_routes)
}
