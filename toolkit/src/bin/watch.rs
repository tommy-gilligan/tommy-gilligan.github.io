use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::io::Read;
use std::path::Path;
use std::process::Child;
use toolkit::shell::{block_spawn, spawn};

use std::sync::{Arc, Mutex, OnceLock};

use futures::FutureExt;
use std::process::ExitStatus;

use notify::Event;
use notify::EventKind::{Modify, Remove};
static GENERATE_CMD: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut path_to_self = std::env::current_exe().unwrap();
    path_to_self.set_file_name("generate");

    // TODO: websocket
    // report file names
    // in generate bin: print status info
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
    let printer = |e: Result<ExitStatus, std::io::Error>| async move {
        match e {
            Ok(e) if e.success() => println!("succeeded"),
            _ => println!("failed"),
        }
    };
    let child_cell: Arc<Mutex<Option<Result<Child, std::io::Error>>>> = Arc::new(Mutex::new(None));
    let for_watcher = child_cell.clone();

    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            ..
        }) = res
        {
            let mut child = for_watcher.lock().unwrap();
            if child.is_none() {
                println!("regenerating due to file change");
                *child = Some(block_spawn(GENERATE_CMD.get().unwrap()));
            // TODO: kill instead of waiting
            } else if let Ok(Some(_)) = child.as_mut().unwrap().as_mut().unwrap().try_wait() {
                println!("regenerating due to file change");
                *child = Some(block_spawn(GENERATE_CMD.get().unwrap()));
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

    let mut build_child = spawn("cargo build --bin generate").then(printer).shared();
    build_child.clone().await;

    for byte in handle.bytes() {
        if byte.unwrap() == b'\n' && build_child.clone().now_or_never().is_some() {
            build_child = spawn("cargo build --bin generate").then(printer).shared();
            build_child.clone().await
        }
    }
}
