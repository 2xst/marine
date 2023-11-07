//TODO opentelemetry

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
            .context("Failed to init tracing subscriber")
    })?;
    Ok(())
}

pub fn init_test_telemetry() {
    TELEMETRY.get_or_init(|| {
        let format = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .pretty();
        let filter = if cfg!(feature = "log-tests") {
            target_filter()
        } else {
            Targets::default()
        };
        tracing_subscriber::registry()
            .with(format)
            .with(filter)
            .init()
    });
}

fn target_filter() -> Targets {
    Targets::new()
        .with_default(Level::WARN)
        .with_target(CRATE_NAME, Level::INFO)
}

#[allow(unused)]
pub(crate) async fn instrument_blocking<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(|| tracing::Span::current().in_scope(f))
        .await
        .context("Failed to spawn blocking task")
}

pub(crate) fn warn<E: std::fmt::Debug>(e: E) -> E {
    tracing::warn!("{e:?}");
    e
}

pub(crate) fn error<E: std::fmt::Debug>(e: E) -> E {
    tracing::error!("{e:?}");
    e
}
