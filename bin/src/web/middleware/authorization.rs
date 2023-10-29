use std::sync::Arc;

use axum::{extract::State, middleware::Next, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use hyper::{Request, StatusCode};
use serde::Serialize;

use foodie_core::ports::incoming::authorization::token_verification_query::TokenVerificationQuery;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub(crate) type DynTokenVerificationService = Arc<dyn TokenVerificationQuery + Sync + Send>;

pub async fn authorization_middleware<B>(
    cookie_jar: CookieJar,
    State(service): State<DynTokenVerificationService>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            let v = request
                .headers()
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|header| header.to_str().ok())
                .and_then(|header_value| {
                    if header_value.starts_with("BEARER") {
                        Some(header_value[7..].to_owned())
                    } else {
                        None
                    }
                });
            v
        });
    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "not logged in".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let user = service.verify_token(token).await;
    let user = user.map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
