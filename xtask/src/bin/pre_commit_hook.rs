use crate::git_directory;

use std::{
    env::{consts::EXE_EXTENSION, current_exe},
    fs::hard_link,
    path::PathBuf,
    process::Command,
};
mod flatten_yaml;
mod fmt;
use git2::Repository;

const HEADER_WITH_BUILD_TIME: &'static str = concat!(
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

    pub fn install(&self) {
        let target = self.pre_commit_path();

        if !target.exists() {
            hard_link(current_exe().unwrap(), &target).unwrap()
        } else {
            // is the file text or binary? + prompt user
        }
    }

    pub fn check_installation(&self) {
        if !self.is_up_to_date() {
            eprintln!("advice on installing");
            std::process::exit(1);
        }
    }

    pub fn run_as_child(&self, search_term: &str) -> bool {
        if self.repository.statuses(None).unwrap().is_empty() {
            let stdout = Command::new(self.pre_commit_path())
                .output()
                .unwrap()
                .stdout;
            String::from_utf8_lossy(&stdout).contains(search_term)
        } else {
            eprintln!("advice on cleaning");
            std::process::exit(1);
        }
    }
}
