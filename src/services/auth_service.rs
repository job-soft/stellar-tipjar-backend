use anyhow::{anyhow, Result};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::models::auth::{AuthResponse, Claims};

/// Access token lifetime: 24 hours.
const ACCESS_TOKEN_SECS: i64 = 60 * 60 * 24;
/// Refresh token lifetime: 7 days.
const REFRESH_TOKEN_SECS: i64 = 60 * 60 * 24 * 7;

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn generate_tokens(username: &str) -> Result<AuthResponse> {
    let secret = jwt_secret();
    let now = Utc::now().timestamp() as usize;

    let access_claims = Claims {
        sub: username.to_owned(),
        kind: "access".to_owned(),
        exp: now + ACCESS_TOKEN_SECS as usize,
        iat: now,
    };

    let refresh_claims = Claims {
        sub: username.to_owned(),
        kind: "refresh".to_owned(),
        exp: now + REFRESH_TOKEN_SECS as usize,
        iat: now,
    };

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_owned(),
    })
}

pub fn validate_token(token: &str, expected_kind: &str) -> Result<Claims> {
    let secret = jwt_secret();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| anyhow!("Invalid token: {}", e))?;

    if token_data.claims.kind != expected_kind {
        return Err(anyhow!("Wrong token kind"));
    }

    Ok(token_data.claims)
}

pub fn hash_password(password: &str) -> Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| anyhow!(e))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    bcrypt::verify(password, hash).map_err(|e| anyhow!(e))
}
