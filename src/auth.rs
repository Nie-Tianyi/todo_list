use crate::models::User;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use axum::extract::FromRequestParts;

// ── JWT secret ───────────────────────────────────

#[cfg(feature = "server")]
const JWT_SECRET: &[u8] = b"todo-list-jwt-secret-change-in-production";

// ── JWT Claims ───────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub exp: usize,
}

// ── AuthSession (server-only extractor) ──────────

/// Extracted from `Authorization: Bearer <token>` header on server functions.
/// The client never sends this — Dioxus fullstack hoists it as a server-only argument.
pub struct AuthSession {
    pub user: User,
}

#[cfg(feature = "server")]
#[derive(Debug)]
pub struct AuthError {
    status: http::StatusCode,
    message: String,
}

#[cfg(feature = "server")]
impl AuthError {
    fn new(status: http::StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

#[cfg(feature = "server")]
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(feature = "server")]
impl std::error::Error for AuthError {}

#[cfg(feature = "server")]
impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}

#[cfg(feature = "server")]
impl<S: Send + Sync> FromRequestParts<S> for AuthSession {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                AuthError::new(
                    http::StatusCode::UNAUTHORIZED,
                    "Missing Authorization header",
                )
            })?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                AuthError::new(
                    http::StatusCode::UNAUTHORIZED,
                    "Invalid Authorization format",
                )
            })?;

        let claims = validate_token(token).map_err(|e| {
            AuthError::new(
                http::StatusCode::UNAUTHORIZED,
                format!("Invalid token: {e}"),
            )
        })?;

        Ok(AuthSession {
            user: User {
                id: claims.sub,
                username: claims.username,
            },
        })
    }
}

// ── JWT helpers (server-only) ────────────────────

#[cfg(feature = "server")]
pub fn create_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 86400; // 24 hours

    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        exp,
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET),
    )
}

#[cfg(feature = "server")]
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)
}

// ── Client-side auth state ───────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub user: User,
    pub token: String,
}

#[derive(Clone, Copy)]
pub struct AuthContext(pub Signal<Option<AuthState>>);

impl AuthContext {
    pub fn is_authenticated(self) -> bool {
        self.0.read().is_some()
    }

    pub fn user(self) -> Option<User> {
        self.0.read().as_ref().map(|s| s.user.clone())
    }

    pub fn username(self) -> Option<String> {
        self.0.read().as_ref().map(|s| s.user.username.clone())
    }

    pub fn login(mut self, state: AuthState) {
        use dioxus::fullstack::set_request_headers;

        let mut headers = http::HeaderMap::new();
        headers.insert(
            http::header::AUTHORIZATION,
            http::HeaderValue::from_str(&format!("Bearer {}", state.token)).unwrap(),
        );
        set_request_headers(headers);
        self.0.set(Some(state));
    }

    pub fn logout(mut self) {
        use dioxus::fullstack::clear_request_headers;
        clear_request_headers();
        self.0.set(None);
    }
}
