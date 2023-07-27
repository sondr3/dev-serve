use crate::cli::{print_completion, Cli};

mod cli;
mod server;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use time::UtcOffset;
use tokio::sync::broadcast;
use tracing_subscriber::{
    fmt::time::OffsetTime, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

#[derive(Debug, Copy, Clone)]
pub enum Event {
    Reload,
    Shutdown,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Cli::parse();

    let offset = UtcOffset::current_local_offset().map_or(UtcOffset::UTC, |o| o);
    let format = time::format_description::parse("[hour]:[minute]:[second]")?;
    let timer = OffsetTime::new(offset, format);
    let fmt = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_timer(timer);
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!("dev_serve={}", if opts.verbose { "debug" } else { "info" }).into()
    });

    tracing_subscriber::registry().with(filter).with(fmt).init();

    if let Some(comp) = opts.completions {
        let mut app = Cli::command();
        print_completion(comp, &mut app);
        return Ok(());
    }

    let root = opts.path.unwrap_or(std::env::current_dir()?);

    let (tx, _rx) = broadcast::channel(100);
    tracing::info!("Serving site at http://localhost:{}/...", opts.port);
    server::create(&root, tx).await?;

    Ok(())
}
