use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Clone, Deserialize, Serialize)]
pub struct UserRecord {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
}

use async_trait::async_trait;
use sqlx::SqlitePool;

use foodie_core::{
    domain::authorization::user::User,
    ports::outgoing::authorization::{
        insert_user_port::{InsertUserError, InsertUserPort},
        query_user_by_email_port::{QueryUserByEmailError, QueryUserByEmailPort},
        query_user_port::{QueryUserError, QueryUserPort},
    },
};

#[derive(Clone, Debug)]
pub struct UserSqliteDS {
    pool: SqlitePool,
}

#[async_trait]
impl QueryUserPort for UserSqliteDS {
    async fn query_user(&self, uuid: uuid::Uuid) -> Result<User, QueryUserError> {
        let uuid = uuid.to_string();
        sqlx::query_as!(UserRecord, r#"SELECT * FROM users WHERE id = ?"#, uuid)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => QueryUserError::UserNotFound,
                _ => QueryUserError::InternalError,
            })
            .map(|result| {
                User::new(
                    uuid::Uuid::parse_str(result.id.as_str()).unwrap(),
                    result.name,
                    result.email,
                    result.password,
                    result.role,
                    result.photo,
                    result.verified,
                )
            })
    }
}

#[async_trait]
impl QueryUserByEmailPort for UserSqliteDS {
    async fn query_user_by_email(&self, email: &str) -> Result<User, QueryUserByEmailError> {
        sqlx::query_as!(UserRecord, r#"SELECT * FROM users WHERE email = ?"#, email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => QueryUserByEmailError::UserNotFound,
                _ => QueryUserByEmailError::InternalError,
            })
            .map(|result| {
                User::new(
                    uuid::Uuid::parse_str(result.id.as_str()).unwrap(),
                    result.name,
                    result.email,
                    result.password,
                    result.role,
                    result.photo,
                    result.verified,
                )
            })
    }
}

#[async_trait]
impl InsertUserPort for UserSqliteDS {
    async fn insert_user(&self, user: User) -> Result<(), InsertUserError> {
        let id = user.id().to_string();
        let name = user.name().to_string();
        let email = user.email().to_string();
        let password = user.password().to_string();
        let role = user.role().to_string();
        let photo = user.photo().to_string();
        let verified = user.verified();
        sqlx::query_as!(
            UserRecord,
            "INSERT INTO users VALUES (?,?,?,?,?,?,?)",
            id,
            name,
            email,
            password,
            role,
            photo,
            verified,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| match e {
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    return InsertUserError::UserAlreadyExists;
                }
                InsertUserError::InternalError
            }
            _ => InsertUserError::InternalError,
        })
    }
}

impl UserSqliteDS {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
