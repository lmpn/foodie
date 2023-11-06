use http::status::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum FoodieAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl FoodieAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            FoodieAppError::NotFound => StatusCode::NOT_FOUND,
            FoodieAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
