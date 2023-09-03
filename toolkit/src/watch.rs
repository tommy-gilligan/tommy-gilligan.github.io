use crate::{article, asset, shell::replacing_spawn};
use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
};
use std::{
    path::{Path, PathBuf},
    process::Child,
};

// FIX: does not give filename on macos
fn print(paths: Vec<PathBuf>) {
    println!("Changes to file(s) detected:");
    for path in paths {
        println!(" - {}", path.display());
    }
}

#[must_use]
pub fn setup(bin_path: &Path, args: Vec<String>) -> notify::FsEventWatcher {
    let mut child: Option<Child> = None;
    let bin_path_buf = bin_path.to_path_buf();
    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            paths,
            ..
        }) = res
        {
            print(paths);
            replacing_spawn(&bin_path_buf, &args, &mut child);
        }
    })
    .unwrap();
    article::watch(&mut watcher);
    asset::watch(&mut watcher);
    watcher
}
