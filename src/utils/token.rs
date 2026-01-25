use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorMessage, HttpError};
const ALGORITHM_SET: Algorithm = Algorithm::HS256;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn create_token(
    user_id: &str,
    secret: &[u8],
    expires_in_minutes: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }

    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::minutes(expires_in_minutes)).timestamp();
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        iat,
        exp,
    };
    let key = &EncodingKey::from_secret(secret);
    let header = &Header::new(ALGORITHM_SET);
    encode(header, &claims, key)
}

pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError> {
    let decoding_key = &DecodingKey::from_secret(secret);
    let validation = &Validation::new(ALGORITHM_SET);
    let decoded = decode::<TokenClaims>(&token.into(), decoding_key, validation);

    match decoded {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(HttpError::new(ErrorMessage::InvalidToken.to_string(), 401)),
    }
}
