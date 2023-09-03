use futures::FutureExt;
use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use std::{
    ffi::{OsStr, OsString},
    io::Read,
    path::Path,
    process::{Child, ExitStatus},
    sync::{Arc, Mutex, OnceLock},
};
use toolkit::shell::{block_spawn, spawn};

static GENERATE_CMD: OnceLock<OsString> = OnceLock::new();
const GENERATE_TARGET_NAME: &str = "generate";

#[tokio::main]
async fn main() {
    let path_to_generate = std::env::current_exe()
        .unwrap()
        .with_file_name(GENERATE_TARGET_NAME);
    let address = toolkit::serve::run().await;
    let mut command_string: OsString = path_to_generate.clone().into();
    command_string.push(OsStr::new(" http://"));
    command_string.push(address.1.ip().to_string());
    command_string.push(OsStr::new(":"));
    command_string.push(address.1.port().to_string());

    // TODO: websocket
    // report file names
    // in generate bin: print status info
    println!("Listening on http://{}", address.1);
    let child_cell: Arc<Mutex<Option<Result<Child, std::io::Error>>>> = Arc::new(Mutex::new(None));
    let for_watcher = child_cell.clone();

    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            paths,
            ..
        }) = res
        {
            let mut child = for_watcher.lock().unwrap();
            if child.is_none() {
                println!("Regenerating due to changes to:");
                for path in paths {
                    println!("- {}", path.display());
                }
                *child = Some(block_spawn(GENERATE_CMD.get().unwrap()));
            // TODO: kill instead of waiting
            } else if let Ok(Some(_)) = child.as_mut().unwrap().as_mut().unwrap().try_wait() {
                println!("Regenerating due to changes to:");
                for path in paths {
                    println!("- {}", path.display());
                }
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
        .watch(&path_to_generate, RecursiveMode::NonRecursive)
        .unwrap();

    GENERATE_CMD.set(command_string).unwrap();
    let printer = |_e: Result<ExitStatus, std::io::Error>| async move {};
    let mut build_child = spawn(GENERATE_TARGET_NAME).then(printer).shared();
    build_child.clone().await;
    for byte in std::io::stdin().lock().bytes() {
        if byte.unwrap() == b'\n' && build_child.clone().now_or_never().is_some() {
            build_child = spawn(GENERATE_TARGET_NAME).then(printer).shared();
            build_child.clone().await
        }
    }
}
