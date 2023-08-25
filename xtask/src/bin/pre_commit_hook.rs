use crate::git_directory;

use std::{
    env::{consts::EXE_EXTENSION, current_exe},
    fs::hard_link,
};
mod flatten_yaml;
mod fmt;

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

pub fn run(force: bool) {
    fmt::run(force);
    flatten_yaml::run(force);
}
