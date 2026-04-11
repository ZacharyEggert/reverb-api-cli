use clap::{ArgMatches, Command};
use reverb_api::RevError;
use std::future::Future;
use std::pin::Pin;

pub mod listings;

/// Trait for service-specific helper commands.
///
/// Helpers add custom subcommands (prefixed with `+`) that go beyond what the
/// schema-driven executor can provide — e.g., multi-step workflows or format translation.
pub trait Helper: Send + Sync {
    /// Add custom subcommands to the clap tree for this resource.
    fn inject_commands(&self, cmd: Command) -> Command;

    /// Handle a matched command. Return `Ok(true)` if handled, `Ok(false)` to fall through
    /// to the schema-driven executor.
    fn handle<'a>(
        &'a self,
        matches: &'a ArgMatches,
        api_key: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, RevError>> + Send + 'a>>;
}

/// Return the helper for a given resource name, if one exists.
pub fn get_helper(resource: &str) -> Option<Box<dyn Helper>> {
    match resource {
        "listings" => Some(Box::new(listings::ListingsHelper)),
        _ => None,
    }
}

// /// Percent-encode a string for safe embedding in a URL path segment.
// pub fn encode_path_segment(s: &str) -> String {
//     percent_encoding::utf8_percent_encode(s, percent_encoding::NON_ALPHANUMERIC).to_string()
// }
