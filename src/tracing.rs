use std::env;

use sentry::{release_name, ClientInitGuard, SessionMode};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init() -> anyhow::Result<ClientInitGuard> {
    let sentry_guard = sentry::init(sentry::ClientOptions {
        auto_session_tracking: true,
        dsn: Some(env::var("SENTRY_DSN")?.parse()?),
        release: release_name!(),
        session_mode: SessionMode::Request,
        ..Default::default()
    });

    let fmt_tracing_layer = tracing_subscriber::fmt::layer().without_time().pretty();
    let tracing_registry = tracing_subscriber::registry()
        .with(fmt_tracing_layer)
        .with(sentry::integrations::tracing::layer())
        .with(EnvFilter::try_from_default_env()?);

    tracing_registry.try_init()?;

    Ok(sentry_guard)
}

#[cfg(test)]
mod tests {
    use std::env;

    use anyhow::{anyhow, Result};
    use dotenvy::dotenv;
    use sentry::ClientInitGuard;
    use tracing::{debug, error, info, trace, warn};

    fn init() -> Result<ClientInitGuard> {
        dotenv()?;
        env::set_var("RUST_LOG", "info");

        crate::tracing::init()
    }

    #[test]
    fn log_message() -> Result<()> {
        let _sentry_guard = init()?;

        let value = 1_u8;
        let err = anyhow!("error");
        trace!(value, "trace");
        debug!(value, "debug");
        info!(value, "info");
        warn!(value, "warn");
        error!(error = &*err, value, "error");

        Ok(())
    }

    #[test]
    fn log_compatibility() -> Result<()> {
        let _sentry_guard = init()?;

        log::info!("Message logged via log crate.");

        Ok(())
    }
}
