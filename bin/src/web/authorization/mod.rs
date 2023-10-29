use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use hyper::Body;

use foodie_core::services::authorization::{
    login_service::LoginService, registration_service::RegistrationService,
    token_verification_service::TokenVerificationService,
};
use foodie_storage::authorization::user_sqlite_ds::UserSqliteDS;

use crate::{configuration::Configuration, state::State};

use self::{
    login_handler::DynLoginService, logout_handler::logout_handler,
    registration_handler::DynRegistrationService,
};

use super::middleware::authorization::{authorization_middleware, DynTokenVerificationService};

pub mod login_handler;
pub mod logout_handler;
pub mod registration_handler;
pub fn router(state: State, configuration: &Configuration) -> Router<(), Body> {
    let storage = UserSqliteDS::new(state.pool());

    let token_verification_service = Arc::new(TokenVerificationService::new(
        storage.clone(),
        configuration.jwt_secret().to_string(),
    )) as DynTokenVerificationService;
    let login_service = Arc::new(LoginService::new(
        storage.clone(),
        configuration.jwt_secret().to_string(),
        configuration.jwt_maxage(),
    )) as DynLoginService;
    let registration_service =
        Arc::new(RegistrationService::new(storage.clone())) as DynRegistrationService;

    let authorization_routes = Router::new()
        .route("/logout", get(logout_handler))
        .route_layer(middleware::from_fn_with_state(
            token_verification_service,
            authorization_middleware,
        ))
        .route(
            "/register",
            post(registration_handler::registration_handler),
        )
        .with_state(registration_service)
        .route("/login", post(login_handler::login_handler))
        .with_state(login_service);

    let authorization_routes = Router::new().nest("/authorization", authorization_routes);
    Router::new().nest("/api/v1", authorization_routes)
}
