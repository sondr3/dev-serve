use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use notify::{
    event::ModifyKind, Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use tokio::sync::broadcast::Sender;

pub fn start_live_reload(
    path: &Path,
    extensions: &[OsString],
    event_tx: &Sender<crate::Event>,
) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path, RecursiveMode::Recursive)?;

    for event in rx {
        match event {
            Ok(event) => {
                if filter_event(&event, extensions) {
                    tracing::info!(
                        "File(s) {:?} changed",
                        strip_prefix_paths(path, &event.paths)?
                    );
                    event_tx.send(crate::Event::Reload)?;
                }
            }
            Err(e) => {
                tracing::error!("watch error: {e:?}");
            }
        }
    }

    Ok(())
}

fn filter_event(event: &Event, extensions: &[OsString]) -> bool {
    match event.kind {
        EventKind::Create(_)
        | EventKind::Modify(ModifyKind::Data(_) | ModifyKind::Name(_))
        | EventKind::Remove(_) => event_has_extension(event, extensions),
        _ => false,
    }
}

fn event_has_extension(event: &Event, extensions: &[OsString]) -> bool {
    event
        .paths
        .iter()
        .any(|p| path_has_extension(p, extensions))
}

fn path_has_extension(path: &Path, extensions: &[OsString]) -> bool {
    path.extension()
        .map_or(false, |e| extensions.contains(&e.to_os_string()))
}

fn strip_prefix_paths(prefix: impl AsRef<Path>, paths: &[PathBuf]) -> Result<Vec<&Path>> {
    paths
        .iter()
        .map(|p| {
            p.strip_prefix(prefix.as_ref().canonicalize()?)
                .context("could not strip prefix")
        })
        .collect()
}
