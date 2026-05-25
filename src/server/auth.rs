use crate::auth::AuthSession;
use crate::models::User;

// ── Password hashing ────────────────────────────────────────────────

pub fn hash_password(password: &str) -> String {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut rand::rngs::OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("failed to hash password")
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

// ── JWT ─────────────────────────────────────────────────────────────

const JWT_SECRET: &[u8] = b"todo-list-jwt-secret-change-in-production";

#[derive(Debug)]
pub struct AuthError {
    status: http::StatusCode,
    message: String,
}

impl AuthError {
    fn new(status: http::StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuthError {}

impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}

impl<S: Send + Sync> axum::extract::FromRequestParts<S> for AuthSession {
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
                gender: None,
                age: None,
                job_title: None,
            },
        })
    }
}

pub fn create_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 86400; // 24 hours

    let claims = crate::auth::Claims {
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

fn validate_token(token: &str) -> Result<crate::auth::Claims, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<crate::auth::Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)
}
