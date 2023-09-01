use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use std::path::Path;
use toolkit::shell::spawn;

use std::io::BufRead;
use std::sync::{Arc, Mutex, OnceLock};

static GENERATE_CMD: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut path_to_self = std::env::current_exe().unwrap();
    path_to_self.set_file_name("generate");

    let address = toolkit::serve::run().await;
    println!("Listening on http://{}", address.1);
    GENERATE_CMD
        .set(
            // todo: use osstr instead
            format!(
                "{} http://{}:{}",
                path_to_self.display(),
                address.1.ip(),
                address.1.port()
            ),
        )
        .unwrap();

    let child_cell = Arc::new(Mutex::new(spawn(GENERATE_CMD.get().unwrap())));
    let for_watcher = child_cell.clone();

    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            ..
        }) = res
        {
            let mut child = for_watcher.lock().unwrap();
            if let Ok(Some(_)) = child.try_wait() {
                println!("regenerating due to file change");
                *child = spawn(GENERATE_CMD.get().unwrap());
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
    watcher
        .watch(&path_to_self, RecursiveMode::NonRecursive)
        .unwrap();

    let mut null = String::new();
    while handle.read_line(&mut null).is_ok() {
        let mut child = child_cell.lock().unwrap();
        if let Ok(Some(_)) = child.try_wait() {
            println!("regenerating due to keypress");
            *child = spawn(GENERATE_CMD.get().unwrap());
        }
    }
}
