use clap::Parser;
use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use std::path::Path;
use std::process::Command;
use std::{thread, time};

// TODO: config should be a superset of serve config
// TODO: what to watch configurable recompile and regenerate vs just regenerate
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value = "articles")]
    pub articles: String,
    #[arg()]
    pub generate: String,
}

fn shell_spawn(command: &str) -> std::process::Child {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .spawn()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("failed to execute process")
    }
}

fn main() {
    let config = Config::parse();

    let mut child = shell_spawn(&config.generate);
    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            ..
        }) = res
        {
            match child.try_wait() {
                Ok(Some(status)) if status.success() => {
                    println!("regenerating");
                    child = shell_spawn(&config.generate);
                }
                _ => (),
            }
        }
    })
    .unwrap();
    watcher
        .watch(Path::new(&config.articles), RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        thread::sleep(time::Duration::from_secs(1));
    }
}
