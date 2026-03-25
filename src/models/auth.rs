use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Register a new creator account with a password.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub wallet_address: String,
    pub password: String,
}

/// Login with username + password.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Returned on successful login or register.
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

/// Refresh access token using a refresh token.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// JWT claims payload.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject — the creator's username.
    pub sub: String,
    /// Token kind: "access" or "refresh".
    pub kind: String,
    /// Expiry as Unix timestamp.
    pub exp: usize,
    /// Issued at as Unix timestamp.
    pub iat: usize,
}
