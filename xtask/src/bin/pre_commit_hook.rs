use crate::git_directory;
use git2::Repository;
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

pub fn run() {
    let repository = Repository::open_from_env().unwrap();
    let head = repository.head().unwrap().peel_to_tree().unwrap();

    fmt::fmt(&repository, &head);
    flatten_yaml::flatten_yaml(&repository, &head);
}
