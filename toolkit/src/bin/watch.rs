use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use std::{
    ffi::{OsStr, OsString},
    io::Read,
    process::Child,
    sync::{Arc, Mutex, OnceLock},
};
use toolkit::shell::{block_spawn, spawn};

static GENERATE_CMD: OnceLock<OsString> = OnceLock::new();
const GENERATE_TARGET_NAME: &str = "generate";

#[tokio::main]
async fn main() {
    toolkit::terminal::setup();
    let address = toolkit::serve::run().await;
    println!("Listening on http://{}", address.1);

    let path_to_generate = std::env::current_exe()
        .unwrap()
        .with_file_name(GENERATE_TARGET_NAME);
    let mut command_string: OsString = path_to_generate.clone().into();
    command_string.push(OsStr::new(" http://"));
    command_string.push(address.1.ip().to_string());
    command_string.push(OsStr::new(":"));
    command_string.push(address.1.port().to_string());
    GENERATE_CMD.set(command_string).unwrap();

    // TODO: websocket
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
                    // FIX: does not give filename on macos
                    println!("- {}", path.display());
                }
                *child = Some(block_spawn(GENERATE_CMD.get().unwrap()));
            // TODO: kill instead of waiting
            } else if let inner_child = child.as_mut().unwrap().as_mut().unwrap() {
                println!("Regenerating due to changes to:");
                for path in paths {
                    // FIX: does not give filename on macos
                    println!("- {}", path.display());
                }
                inner_child.kill().unwrap();
                inner_child.wait().unwrap();
                *child = Some(block_spawn(GENERATE_CMD.get().unwrap()));
            }
        }
    })
    .unwrap();
    toolkit::article::watch(&mut watcher);
    toolkit::asset::watch(&mut watcher);
    watcher
        .watch(&path_to_generate, RecursiveMode::NonRecursive)
        .unwrap();

    let mut build_child = spawn(GENERATE_TARGET_NAME).unwrap();
    for _byte in std::io::stdin().lock().bytes() {
        build_child.kill().unwrap();
        build_child.wait().unwrap();
        build_child = spawn(GENERATE_TARGET_NAME).unwrap();
    }
    toolkit::terminal::cleanup();
}
