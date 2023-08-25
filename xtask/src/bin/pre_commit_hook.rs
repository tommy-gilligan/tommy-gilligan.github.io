use crate::git_directory;
use git2::{Repository, Tree};
use std::env::{consts::EXE_EXTENSION, current_exe, var};
use std::ffi::OsStr;
use std::fs::hard_link;
use std::path::Path;
use std::process::Command;

fn fmt(repository: &Repository, head: &Tree) {
    let rust_extension = OsStr::new("rs");

    let mut staged = repository
        .diff_tree_to_index(Some(head), None, None)
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

    if staged_rust_files.peek().is_some() {
        assert!(
            Command::new(var("CARGO").unwrap_or("cargo".to_owned()))
                .arg("fmt")
                .arg("--check")
                .arg("--")
                .args(staged_rust_files)
                .current_dir(git_directory())
                .status()
                .unwrap()
                .success(),
            "Aborting commit due to bad formatting"
        )
    }
}

fn flatten_yaml(repository: &Repository, _head: &Tree) {
    let ci_yaml = Path::new("ci.yml");
    let target = Path::new(".github/workflows/ci.yml");
    assert!(ci_yaml.exists());
    assert!(target.exists());

    if repository
        .status_file(ci_yaml)
        .unwrap()
        .contains(git2::Status::INDEX_MODIFIED)
    {
        crate::flatten_yaml::check(ci_yaml, target);
    }
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

    fmt(&repository, &head);
    flatten_yaml(&repository, &head);
}
