use cfg_if::cfg_if;
use foodie_core::{ports::outgoing::authorization::query_user_port::QueryUserPort, domain::authorization::filtered_user::FilteredUser};
use foodie_storage::authorization::user_sqlite_ds::UserSqliteDS;
use leptos::*;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::collections::HashSet;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    use bcrypt::{hash, verify, DEFAULT_COST};
    use crate::todo::{pool, auth};
    pub type AuthSession = axum_session_auth::AuthSession<AuthenticatedUser, String, SessionSqlitePool, UserSqliteDS>;
}}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
}

impl AuthenticatedUser {
    pub fn new(id: String, name: String, email: String, role: String, photo: String, verified: bool) -> Self { Self { id, name, email, role, photo, verified } }
}

impl Default for AuthenticatedUser {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            email: Default::default(),
            role: Default::default(),
            photo: Default::default(), 
            verified: Default::default() 
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use async_trait::async_trait;

    /*
    impl User {
        pub async fn get(id: i64, pool: &dyn QueryUserPort) -> Option<Self> {
            todo!()
            // let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
            //     .bind(id)
            //     .fetch_one(pool)
            //     .await
            //     .ok()?;

            // //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            // let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            //     "SELECT token FROM user_permissions WHERE user_id = ?;",
            // )
            // .bind(id)
            // .fetch_all(pool)
            // .await
            // .ok()?;

            // Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub async fn get_from_username(name: String, pool: &SqlitePool) -> Option<Self> {
            todo!()
            // let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = ?")
            //     .bind(name)
            //     .fetch_one(pool)
            //     .await
            //     .ok()?;

            // //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            // let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            //     "SELECT token FROM user_permissions WHERE user_id = ?;",
            // )
            // .bind(sqluser.id)
            // .fetch_all(pool)
            // .await
            // .ok()?;

            // Some(sqluser.into_user(Some(sql_user_perms)))
        }
    }
    */
    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[async_trait]
    impl Authentication<AuthenticatedUser, String, UserSqliteDS> for AuthenticatedUser {
        async fn load_user(userid: String, pool: Option<&UserSqliteDS>) -> Result<AuthenticatedUser, anyhow::Error> {
            let pool = pool.ok_or_else(|| anyhow::anyhow!("Cannot get user")).unwrap() as &dyn QueryUserPort;
            pool.query_user(Uuid::parse_str(&userid)?).await.map(
                |user| 
                    AuthenticatedUser::new(
                        user.id().to_owned(),
                        user.name().to_owned(),
                        user.email().to_owned(),
                        user.role().to_owned(),
                        user.photo().to_owned(),
                        user.verified().to_owned(),
                    )
            )
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
    let pool = pool()?;
    let auth = auth()?;

    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| {
            ServerFnError::ServerError("User does not exist.".into())
        })?;

    match verify(password, &user.password)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    sqlx::query("INSERT INTO users (username, password) VALUES (?,?)")
        .bind(username.clone())
        .bind(password_hashed)
        .execute(&pool)
        .await?;

    let user =
        User::get_from_username(username, &pool)
            .await
            .ok_or_else(|| {
                ServerFnError::ServerError(
                    "Signup failed: User does not exist.".into(),
                )
            })?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect("/");

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
