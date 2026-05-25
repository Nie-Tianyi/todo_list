use crate::models::User;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// ── JWT Claims ───────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)] // constructed only in server.rs
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub exp: usize,
}

// ── AuthSession (extractor target) ──────────────

/// The `FromRequestParts` implementation is in `server.rs`.
#[allow(dead_code)] // constructed only in server.rs
pub struct AuthSession {
    pub user: User,
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

    pub fn update_user(mut self, user: User) {
        if let Some(ref mut state) = *self.0.write() {
            state.user = user;
        }
    }

    pub fn logout(mut self) {
        use dioxus::fullstack::clear_request_headers;
        clear_request_headers();
        self.0.set(None);
    }
}
