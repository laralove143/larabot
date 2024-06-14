use anyhow::Result;
use tracing::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub mod interaction;
pub mod localization;
pub mod option_ext;

pub fn init_tracing() -> Result<()> {
    let fmt_tracing_layer = tracing_subscriber::fmt::layer().without_time().pretty();
    let tracing_registry = tracing_subscriber::registry()
        .with(fmt_tracing_layer)
        .with(EnvFilter::try_from_default_env()?);

    if let Ok(layer) = tracing_journald::layer() {
        tracing_registry.with(layer).try_init()?;
    } else {
        warn!("not using journald for tracing");
        tracing_registry.try_init()?;
    }

    Ok(())
}
