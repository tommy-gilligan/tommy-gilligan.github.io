use notify::RecursiveMode;
use notify::Watcher;
use std::{env::var, io::Read};
use toolkit::{serve, shell::replacing_spawn, terminal, watch};

const GENERATE_TARGET_NAME: &str = "generate";
const CARGO_ARGS: [&str; 3] = ["build", "--bin", GENERATE_TARGET_NAME];

fn path_to_cargo() -> String {
    var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

#[tokio::main]
async fn main() {
    terminal::setup();
    // TODO: websocket to signal clients to refresh
    let server = serve::run().await;
    println!("Listening on http://{}", server.1);

    let generate_path = std::env::current_exe()
        .unwrap()
        .with_file_name(GENERATE_TARGET_NAME);
    let generate_args = vec![format!(
        "http://{}:{}",
        &server.1.ip().to_string(),
        &server.1.port().to_string()
    )];

    let mut watcher = watch::setup(&generate_path, generate_args);
    watcher
        .watch(&generate_path, RecursiveMode::NonRecursive)
        .unwrap();

    let mut cargo_child = None;
    replacing_spawn(&path_to_cargo(), CARGO_ARGS, &mut cargo_child);
    for _byte in std::io::stdin().lock().bytes() {
        replacing_spawn(&path_to_cargo(), CARGO_ARGS, &mut cargo_child);
    }
    terminal::cleanup();
}
