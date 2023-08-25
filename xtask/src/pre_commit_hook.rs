use std::env::consts::EXE_EXTENSION;
use std::env::current_exe;

use std::fs::hard_link;

pub fn install() {
    let target = git2::Repository::open_from_env()
        .unwrap()
        .workdir()
        .unwrap()
        .join(".git")
        .join("hooks")
        .join("pre-commit")
        .with_extension(EXE_EXTENSION);
    let source = current_exe().unwrap();

    if !target.exists() {
        hard_link(source, &target).unwrap()
    }
}

pub fn run() {
    println!("pre-commit hook");
}
