use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String, // subject
    pub iat: usize,  // issued at time
    pub exp: usize,  // expiration
    pub role: String,
}

impl TokenClaims {
    pub fn new(sub: String, maxage: i64, role: String) -> Self {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::seconds(maxage)).timestamp() as usize;
        Self {
            sub,
            iat,
            exp,
            role,
        }
    }
}
