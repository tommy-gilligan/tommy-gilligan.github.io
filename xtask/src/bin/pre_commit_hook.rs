use std::{
    env::{consts::EXE_EXTENSION, current_exe},
    path::{Path, PathBuf},
    process::Command,
};
mod flatten_yaml;
mod fmt;
use git2::Repository;

const HEADER_WITH_BUILD_TIME: &str = concat!(
    "Running pre-commit from ",
    env!("CARGO_CRATE_NAME"),
    " built at ",
    env!("BUILD_TIME")
);

pub fn run(force: bool) {
    println!("{}", HEADER_WITH_BUILD_TIME);
    fmt::run(force);
    flatten_yaml::run(force);
}

pub struct PreCommitHook {
    repository: Repository,
}

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn is_executable(path: &Path) -> bool {
    match path.canonicalize().and_then(|f| f.metadata()) {
        Ok(metadata) if metadata.is_file() => {
            if cfg!(unix) {
                (metadata.permissions().mode() & 0o111) != 0
            } else {
                true
            }
        }
        _ => false,
    }
}

impl PreCommitHook {
    pub fn new() -> Self {
        Self {
            repository: Repository::open_from_env().unwrap(),
        }
    }

    fn hooks_path(&self) -> PathBuf {
        self.repository
            .config()
            .unwrap()
            .get_path("core.hooksPath")
            .unwrap_or(self.repository.path().join("hooks"))
    }

    fn pre_commit_path(&self) -> PathBuf {
        self.hooks_path()
            .join("pre-commit")
            .with_extension(EXE_EXTENSION)
    }

    fn is_up_to_date(&self) -> bool {
        self.run_as_child(HEADER_WITH_BUILD_TIME)
    }

    pub fn check_installation(&self) {
        if !self.is_up_to_date() {
            eprintln!("advice on installing {:?}", current_exe().unwrap());
            std::process::exit(1);
        }
    }

    pub fn run_as_child(&self, search_term: &str) -> bool {
        let mut status_options = git2::StatusOptions::new();
        status_options.include_ignored(false);
        if !self
            .repository
            .statuses(Some(&mut status_options))
            .unwrap()
            .is_empty()
        {
            eprintln!("advice on cleaning");
            std::process::exit(1);
        }
        let path = self.pre_commit_path();
        if !is_executable(&path) {
            eprintln!("not valid exectuable");
            std::process::exit(1);
        }
        let stdout = Command::new(path).output().unwrap().stdout;
        String::from_utf8_lossy(&stdout).contains(search_term)
    }
}
