use tracing::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init() -> anyhow::Result<()> {
    let fmt_tracing_layer = tracing_subscriber::fmt::layer().without_time().pretty();
    let tracing_registry = tracing_subscriber::registry()
        .with(fmt_tracing_layer)
        .with(EnvFilter::try_from_default_env()?);

    if let Ok(layer) = tracing_journald::layer() {
        tracing_registry.with(layer).try_init()?;
    } else {
        warn!("Not using journald for tracing.");
        tracing_registry.try_init()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use anyhow::Result;
    use tracing::{debug, error, info, trace, warn};

    fn init() -> Result<()> {
        env::set_var("RUST_LOG", "trace");
        crate::tracing::init()?;

        Ok(())
    }

    #[test]
    fn log_message() -> Result<()> {
        init()?;

        let value = 1_u8;
        trace!(value, "trace");
        debug!(value, "debug");
        info!(value, "info");
        warn!(value, "warn");
        error!(value, "error");

        Ok(())
    }

    #[test]
    fn log_compatibility() -> Result<()> {
        init()?;

        log::info!("Message logged via log crate.");

        Ok(())
    }
}
