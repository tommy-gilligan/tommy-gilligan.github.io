use std::env::{consts::EXE_EXTENSION, current_exe, var};
use std::process::Command;
use std::fs::hard_link;
use crate::git_directory;

fn fmt() {
    assert!(
        Command::new(var("CARGO").unwrap_or("cargo".to_owned()))
            .arg("fmt")
            .current_dir(git_directory())
            .status()
            .unwrap()
            .success()
        )
}

pub fn install() {
    let target = git_directory()
        .join(".git")
        .join("hooks")
        .join("pre-commit")
        .with_extension(EXE_EXTENSION);

    if !target.exists() {
        hard_link(current_exe().unwrap(), &target).unwrap()
    }
}

pub fn run() {
    fmt();
}
