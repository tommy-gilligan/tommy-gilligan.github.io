use crate::git_directory;
use git2::Repository;
use std::env::var;
use std::ffi::OsStr;
use std::process::Command;

pub fn run(force: bool) {
    let repository = Repository::open_from_env().unwrap();
    let head = repository.head().unwrap().peel_to_tree().unwrap();

    let rust_extension = OsStr::new("rs");

    let mut staged = repository
        .diff_tree_to_index(Some(&head), None, None)
        .unwrap();
    staged.find_similar(None).unwrap();
    let mut staged_rust_files = staged
        .deltas()
        .filter_map(|diff_delta| {
            let path = diff_delta.new_file().path().unwrap();
            if path.extension() == Some(rust_extension) {
                Some(path)
            } else {
                None
            }
        })
        .peekable();

    if staged_rust_files.peek().is_some() || force {
        let mut command = Command::new(var("CARGO").unwrap_or("cargo".to_owned()));
        command
            .arg("fmt")
            .arg("--check")
            .current_dir(git_directory());
        if !force {
            command.arg("--").args(staged_rust_files);
        }
        if !command.status().unwrap().success() {
            std::process::exit(1);
        }
    }
}
