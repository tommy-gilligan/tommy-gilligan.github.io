use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use std::path::Path;

// TODO: config should be a superset of serve config
// TODO: what to watch configurable recompile and regenerate vs just regenerate
#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "articles")]
    pub articles: String,
}

pub async fn watch(config: &Args) {
    let mut child = crate::generate::run();

    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            paths: _,
            ..
        }) = res
        {
            let status = child.try_wait().unwrap().unwrap();
            if status.success() {
                child = crate::generate::run();
            } else {
                panic!()
            }
        }
    })
    .unwrap();
    watcher
        .watch(Path::new(&config.articles), RecursiveMode::NonRecursive)
        .unwrap();

    crate::serve::serve(&crate::serve::Args::default())
        .await
        .unwrap();
}
