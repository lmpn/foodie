use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};
cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::server::{context_authorization_service, context_authorization_session_service};
use async_trait::async_trait;
use leptos::{logging::log};
use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
use foodie_storage::authorization::user_sqlite_ds::UserSqliteDS;
pub type AuthorizationSession = axum_session_auth::AuthSession<AuthenticatedUser, String, SessionSqlitePool, UserSqliteDS>;
use foodie_core::{
    ports::outgoing::authorization::{query_user_port::QueryUserPort},
};
}}

/**
 * Note: there is an hard dependency on the data model meaning that
 * if the model changes this will have to change
 *
 * TODO: assess if this is ok because this is using SSR
 * or if it should be broken with mapper
 * or reuse the extend the foodie_core::ports::incoming::authorization module
*/
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
}

impl AuthenticatedUser {
    pub fn new(id: String, name: String, email: String, role: String, verified: bool) -> Self {
        Self {
            id,
            name,
            email,
            role,
            verified,
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    #[async_trait]
    impl Authentication<AuthenticatedUser, String, UserSqliteDS> for AuthenticatedUser {
        async fn load_user(userid: String, pool: Option<&UserSqliteDS>) -> Result<AuthenticatedUser, anyhow::Error> {
            let pool = pool.ok_or_else(|| anyhow::anyhow!("Cannot get user")).unwrap() as &dyn QueryUserPort;
            pool.query_user(uuid::Uuid::parse_str(&userid)?).await.map(
                |user|
                    AuthenticatedUser::new(
                        user.id().to_string(),
                        user.name().to_owned(),
                        user.email().to_owned(),
                        user.role().to_owned(),
                        user.verified().to_owned(),
                    )
            ).map_err(Into::into)
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<UserSqliteDS> for AuthenticatedUser {
        async fn has(&self, _perm: &str, _: &Option<&UserSqliteDS>) -> bool {
            // self.permissions.contains(perm)
            //TODO verify permissions
            true
        }
    }
}
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<AuthenticatedUser>, ServerFnError> {
    let session = context_authorization_session_service()?;

    Ok(session.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use foodie_core::ports::incoming::authorization::login_command::{LoginCommand, Request};
    let session = context_authorization_session_service()?;
    let service = &context_authorization_service()? as &dyn LoginCommand;
    let token = service.login(Request::new(username, password)).await;
    match token {
        Ok(token) => {
            session.login_user(token.sub);
            session.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        Err(e) => {
            log!("kind:{}", e);
            Err(ServerFnError::ServerError(
                "Error logging in user.".to_string(),
            ))
        }
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use foodie_core::ports::incoming::authorization::registration_command::{
        RegistrationCommand, Request,
    };
    let session = context_authorization_session_service()?;
    let registration_service = &context_authorization_service()? as &dyn RegistrationCommand;
    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Error password mismatch.".to_string(),
        ));
    }
    let token = registration_service
        .register(Request::new(username.clone(), username, password))
        .await;
    match token {
        Ok(token) => {
            session.login_user(token.sub);
            session.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        Err(_) => Err(ServerFnError::ServerError(
            "Error registering in user.".to_string(),
        )),
    }
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let session = context_authorization_session_service()?;
    session.logout_user();
    leptos_axum::redirect("/");
    Ok(())
}
