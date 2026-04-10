use tracing_subscriber::{fmt, EnvFilter};

/// Initialize structured logging.
///
/// Controlled by:
///   REVERB_CLI_LOG      — log level filter (e.g. "revcli=debug"). Off by default.
///   REVERB_CLI_LOG_FILE — directory for daily-rotated JSON log files. Off by default.
pub fn init() {
    let filter = EnvFilter::try_from_env("REVERB_CLI_LOG")
        .unwrap_or_else(|_| EnvFilter::new("off"));

    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}
