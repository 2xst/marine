use anyhow::Context;
use once_cell::sync::OnceCell;
use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt::format::FmtSpan, prelude::*};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

static TELEMETRY: OnceCell<()> = OnceCell::new();

pub fn init_telemetry() -> anyhow::Result<()> {
    TELEMETRY.get_or_try_init(|| {
        let format = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .pretty();
        tracing_subscriber::registry()
            .with(format)
            .with(target_filter())
            .try_init()
            .context("failed to init tracing subscriber")
    })?;
    Ok(())
}

fn target_filter() -> Targets {
    Targets::new()
        .with_default(Level::WARN)
        .with_target(CRATE_NAME, Level::INFO)
}

pub async fn _instrument_blocking<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(|| tracing::Span::current().in_scope(f))
        .await
        .context("failed to spawn blocking task")
}

pub fn warn<E: std::fmt::Debug>(e: E) -> E {
    tracing::warn!("{e:?}");
    e
}

pub fn error<E: std::fmt::Debug>(e: E) -> E {
    tracing::error!("{e:?}");
    e
}
