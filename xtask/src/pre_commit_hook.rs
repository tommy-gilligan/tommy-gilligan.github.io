use crate::git_directory;
use git2::Repository;
use std::collections::HashSet;
use std::env::{consts::EXE_EXTENSION, current_exe, var};
use std::ffi::OsStr;
use std::fs::hard_link;
use std::path::PathBuf;
use std::process::Command;

fn fmt<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    assert!(
        Command::new(var("CARGO").unwrap_or("cargo".to_owned()))
            .arg("fmt")
            .arg("--")
            .args(args)
            .current_dir(git_directory())
            .status()
            .unwrap()
            .success(),
        "Aborting commit due to bad formatting"
    );
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
    let repository = Repository::open_from_env().unwrap();
    let head = repository.head().unwrap().peel_to_tree().unwrap();
    let staged = repository
        .diff_tree_to_index(Some(&head), None, None)
        .unwrap();

    let staged_files: HashSet<PathBuf> = staged
        .deltas()
        .map(|diff_delta| diff_delta.new_file().path().unwrap().to_path_buf())
        .collect();

    let rust_extension = OsStr::new("rs");
    let rust_files = staged_files
        .iter()
        .filter(|file| file.extension() == Some(rust_extension));

    fmt(rust_files);

    // let unstaged = repository.diff_tree_to_workdir(Some(&head), None).unwrap();

    // let unstaged_files: HashSet<PathBuf> = unstaged
    //     .deltas()
    //     .map(|diff_delta| diff_delta.new_file().path().unwrap().to_path_buf())
    //     .collect();

    // let intersection = staged_files.intersection(&unstaged_files);
    // if intersection.clone().count() != 0 {
    //     eprintln!("Aborting commit because these files have changed");
    //     for file in intersection {
    //         eprintln!("{}", file.display());
    //     }
    //     std::process::exit(1);
    // }
}
