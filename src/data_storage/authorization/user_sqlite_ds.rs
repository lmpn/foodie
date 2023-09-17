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
use tracing::error;

use crate::services::{
    domain::user::User,
    ports::outgoing::{
        insert_user_port::{InsertUserError, InsertUserPort},
        query_user_by_email_port::{QueryUserByEmailError, QueryUserByEmailPort},
        query_user_port::{QueryUserError, QueryUserPort},
    },
};

impl From<sqlx::Error> for QueryUserError {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value);
        match value {
            sqlx::Error::RowNotFound => QueryUserError::UserNotFound,
            _ => QueryUserError::InternalError,
        }
    }
}

impl From<sqlx::Error> for QueryUserByEmailError {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value);
        match value {
            sqlx::Error::RowNotFound => QueryUserByEmailError::UserNotFound,
            _ => QueryUserByEmailError::InternalError,
        }
    }
}

// impl From<sqlx::Error> for UpdateUserError {
//     fn from(value: sqlx::Error) -> Self {
//         match value {
//             sqlx::Error::RowNotFound => UpdateUserError::RecordNotFound,
//             _ => UpdateUserError::InternalError,
//         }
//     }
// }

// impl From<sqlx::Error> for DeleteUserError {
//     fn from(value: sqlx::Error) -> Self {
//         match value {
//             sqlx::Error::RowNotFound => DeleteUserError::RecordNotFound,
//             _ => DeleteUserError::InternalError,
//         }
//     }
// }

impl From<sqlx::Error> for InsertUserError {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value);
        InsertUserError::InternalError
    }
}

#[derive(Clone)]
pub struct UserSqliteDS {
    pool: SqlitePool,
}

// #[async_trait]
// impl UpdateUserPort for UserSqliteDS {
//     async fn update_recipe(
//         &self,
//         record: User,
//         deleted_ingredients: Vec<uuid::Uuid>,
//     ) -> Result<(), UpdateUserError> {
//         let transaction = self.pool.begin().await?;
//         let name = record.name().to_string();
//         let method = record.method();
//         let image = record.image();
//         let uuid = record.uuid().to_string();
//         let result = sqlx::query!(
//             r#" UPDATE recipe SET name = ?, method = ?, image = ?  WHERE uuid = ?  "#,
//             name,
//             method,
//             image,
//             uuid
//         )
//         .execute(&self.pool)
//         .await
//         .map(|_e| ())
//         .map_err(|e| e.into());
//         if result.is_err() {
//             transaction.rollback().await?;
//             return result;
//         }
//         for uuid in deleted_ingredients {
//             let uuid = uuid.to_string();
//             let result = sqlx::query!(r#"DELETE FROM ingredient WHERE uuid = ?"#, uuid)
//                 .execute(&self.pool)
//                 .await
//                 .map(|_e| ())
//                 .map_err(|e| e.into());
//             if result.is_err() {
//                 transaction.rollback().await?;
//                 return result;
//             }
//         }
//         for item in record.ingredients() {
//             let uuid = item.uuid().to_string();
//             let name = item.name();
//             let amount = item.amount();
//             let unit = item.unit();
//             let result = sqlx::query!(
//                 "INSERT INTO ingredient (uuid, name, amount, unit)
//                           VALUES (?,?,?,?)
//                           ON CONFLICT (uuid)
//                           DO UPDATE SET name = ?, amount = ?, unit = ?",
//                 uuid,
//                 name,
//                 amount,
//                 unit,
//                 name,
//                 amount,
//                 unit
//             )
//             .execute(&self.pool)
//             .await
//             .map(|_e| ())
//             .map_err(|e| e.into());
//             if result.is_err() {
//                 transaction.rollback().await?;
//                 return result;
//             }
//         }
//         transaction.commit().await?;
//         Ok(())
//     }
// }

#[async_trait]
impl QueryUserPort for UserSqliteDS {
    async fn query_user(&self, uuid: uuid::Uuid) -> Result<User, QueryUserError> {
        let uuid = uuid.to_string();
        let result = sqlx::query_as!(UserRecord, r#"SELECT * FROM users WHERE id = ?"#, uuid)
            .fetch_one(&self.pool)
            .await?;

        Ok(User::new(
            uuid::Uuid::parse_str(result.id.as_str()).unwrap(),
            result.name,
            result.email,
            result.password,
            result.role,
            result.photo,
            result.verified,
        ))
    }
}

#[async_trait]
impl QueryUserByEmailPort for UserSqliteDS {
    async fn query_user_by_email(&self, email: String) -> Result<User, QueryUserByEmailError> {
        let result = sqlx::query_as!(UserRecord, r#"SELECT * FROM users WHERE email = ?"#, email)
            .fetch_one(&self.pool)
            .await?;

        Ok(User::new(
            uuid::Uuid::parse_str(result.id.as_str()).unwrap(),
            result.name,
            result.email,
            result.password,
            result.role,
            result.photo,
            result.verified,
        ))
    }
}
// #[async_trait]
// impl DeleteUserPort for UserSqliteDS {
//     async fn delete_recipe(&self, uuid: Uuid) -> Result<(), DeleteUserError> {
//         let mut builder = QueryBuilder::new("DELETE FROM recipe WHERE uuid = ");
//         let query = builder.push_bind(uuid.to_string()).build();
//         query
//             .execute(&self.pool)
//             .await
//             .map(|_v| ())
//             .map_err(|e| e.into())
//     }
// }

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
        .await?;
        Ok(())
    }
}

impl UserSqliteDS {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
