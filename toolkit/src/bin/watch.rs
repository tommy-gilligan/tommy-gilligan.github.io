use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use std::path::Path;
use toolkit::shell::spawn;

#[tokio::main]
async fn main() {
    let address = toolkit::serve::run().await;
    println!("Listening on http://{}", address.1);

    let generate_cmd = format!(
        "cargo run --bin generate -- http://{}:{}",
        address.1.ip(),
        address.1.port()
    );

    let mut child = spawn(&generate_cmd);
    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            ..
        }) = res
        {
            if let Ok(Some(_)) = child.try_wait() {
                child = spawn(&generate_cmd);
            }
        }
    })
    .unwrap();
    watcher
        .watch(Path::new(toolkit::ARTICLES), RecursiveMode::Recursive)
        .unwrap();
    watcher
        .watch(Path::new(toolkit::ASSETS), RecursiveMode::Recursive)
        .unwrap();

    address.0.await.unwrap();
}
