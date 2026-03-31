//! Logging initialization for the jyotish crate.
//!
//! Requires the `logging` feature. Uses the `JYOTISH_LOG` environment variable
//! to control filter levels (defaults to `warn`).

/// Initialize tracing with the `JYOTISH_LOG` environment variable.
///
/// Returns `Ok(())` on success, or an error if the global subscriber has
/// already been set.
pub fn try_init() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use tracing_subscriber::EnvFilter;
    let filter =
        EnvFilter::try_from_env("JYOTISH_LOG").unwrap_or_else(|_| EnvFilter::new("warn"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
}
