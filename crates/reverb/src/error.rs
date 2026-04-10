use serde::Serialize;
use thiserror::Error;

/// Structured error type for all Reverb CLI operations.
///
/// Exit codes:
///   0 — success
///   1 — API error (4xx/5xx from Reverb)
///   2 — authentication error (missing/invalid API key)
///   3 — validation error (bad user input)
///   4 — schema error (unknown resource or method)
///   5 — unexpected error
#[derive(Debug, Error, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RevError {
    #[error("API error {code}: {message}")]
    Api { code: u16, message: String },

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl RevError {
    pub fn exit_code(&self) -> i32 {
        match self {
            RevError::Api { .. } => 1,
            RevError::Auth(_) => 2,
            RevError::Validation(_) => 3,
            RevError::Schema(_) => 4,
            RevError::Other(_) => 5,
        }
    }
}
