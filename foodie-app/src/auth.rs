use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos::{logging::log};
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    use crate::todo::{ auth};
    use foodie_storage::authorization::user_sqlite_ds::UserSqliteDS;
    pub type AuthSession = axum_session_auth::AuthSession<AuthenticatedUser, String, SessionSqlitePool, UserSqliteDS>;
}}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl Default for AuthenticatedUser {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            email: Default::default(),
            role: Default::default(),
            verified: Default::default(),
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use async_trait::async_trait;
    use foodie_core::{
        ports::outgoing::authorization::{
            query_user_by_email_port::QueryUserByEmailPort, query_user_port::QueryUserPort,
        },
        services::authorization::service::AuthorizationService,
    };

    fn authorization_service() -> Result<AuthorizationService<UserSqliteDS>, ServerFnError> {
        use_context::<AuthorizationService<UserSqliteDS>>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
    }

    impl AuthenticatedUser {
        pub async fn get(id: String, pool: &dyn QueryUserPort) -> Option<Self> {
            let user = pool.query_user(uuid::Uuid::parse_str(&id).unwrap_or_default()).await.ok()?;
            Some(AuthenticatedUser::new(
                user.id().to_string(),
                user.name().to_string(),
                user.email().to_string(),
                user.role().to_string(),
                user.verified()
            ))
        }

        pub async fn get_from_username(name: String, pool: &dyn QueryUserByEmailPort) -> Option<Self> {
            let user = pool.query_user_by_email(&name).await.ok()?;
            Some(AuthenticatedUser::new(
                user.id().to_string(),
                user.name().to_string(),
                user.email().to_string(),
                user.role().to_string(),
                user.verified()
            ))
        }
    }

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
        async fn has(&self, perm: &str, _: &Option<&UserSqliteDS>) -> bool {
            // self.permissions.contains(perm)
            true
        }
    }
}
}

#[server(Foo, "/api")]
pub async fn foo() -> Result<String, ServerFnError> {
    Ok(String::from("Bar!"))
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<AuthenticatedUser>, ServerFnError> {
    let auth = auth()?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use foodie_core::ports::incoming::authorization::login_command::{LoginCommand, Request};
    let session = auth()?;
    let service = &authorization_service()? as &dyn LoginCommand;
    let token = service.login(Request::new(username, password)).await;
    log!("login here");
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
    let session = auth()?;
    let registration_service = &authorization_service()? as &dyn RegistrationCommand;
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
    let session = auth()?;
    session.logout_user();
    leptos_axum::redirect("/");
    Ok(())
}
