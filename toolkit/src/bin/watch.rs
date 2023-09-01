use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use std::path::Path;
use toolkit::shell::spawn;

const GENERATE_CMD: &str = "cargo run --bin generate -- http://localhost:9231";

#[tokio::main]
async fn main() {
    let mut child = spawn(GENERATE_CMD);
    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            ..
        }) = res
        {
            match child.try_wait() {
                Ok(Some(status)) if status.success() => {
                    println!("regenerating");
                    child = spawn(GENERATE_CMD);
                }
                _ => (),
            }
        }
    })
    .unwrap();
    watcher
        .watch(Path::new(toolkit::ARTICLES), RecursiveMode::Recursive)
        .unwrap();
    watcher
        .watch(Path::new(toolkit::STYLE), RecursiveMode::NonRecursive)
        .unwrap();

    let address = toolkit::serve::run().await;
    println!("Listening on http://{}", address.1);
    address.0.await.unwrap();
}
