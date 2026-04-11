use std::sync::OnceLock;
use std::time::Duration;

use reqwest::{Client, Response, StatusCode};
use tracing::warn;

use crate::error::RevError;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent(concat!("revcli/", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client")
    })
}

/// Execute a request with exponential-backoff retry on 429 and transient errors.
pub async fn execute_with_retry(
    build_request: impl Fn() -> reqwest::RequestBuilder,
) -> Result<Response, RevError> {
    let mut delay = Duration::from_secs(1);
    let max_delay = Duration::from_secs(60);
    let max_attempts = 5;

    for attempt in 1..=max_attempts {
        let resp = build_request()
            .send()
            .await
            .map_err(|e| RevError::Other(e.into()))?;

        if resp.status() == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = resp
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .map(Duration::from_secs)
                .unwrap_or(delay);

            if attempt < max_attempts {
                warn!(attempt, ?retry_after, "rate limited, retrying");
                tokio::time::sleep(retry_after).await;
                delay = (delay * 2).min(max_delay);
                continue;
            }
        }

        return Ok(resp);
    }

    Err(RevError::Api {
        code: 429,
        message: "rate limit exceeded after retries".into(),
    })
}
