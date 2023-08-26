use git2::Repository;
use std::env::{args_os, var};
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, ExitStatus};

mod flatten_yaml;
mod pre_commit_hook;

fn git_directory() -> std::path::PathBuf {
    Repository::open_from_env()
        .unwrap()
        .workdir()
        .unwrap()
        .to_path_buf()
}

fn cargo<I, S>(package: &str, args: I) -> ExitStatus
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new(var("CARGO").unwrap_or("cargo".to_owned()))
        .arg("run")
        .arg("--package")
        .arg(package)
        .arg("--")
        .args(args)
        .current_dir(git_directory())
        .status()
        .unwrap();

    std::process::exit(status.code().unwrap());
}

fn cargo_self<I, S>(package: &str, args: I) -> ExitStatus
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new(var("CARGO").unwrap_or("cargo".to_owned()))
        .arg("run")
        .arg("--bin")
        .arg(package)
        .arg("--")
        .args(args)
        .current_dir(git_directory())
        .status()
        .unwrap();

    std::process::exit(status.code().unwrap());
}

fn setup_environment() {
    pre_commit_hook::PreCommitHook::new().install();
}

fn check_environment() {
    pre_commit_hook::PreCommitHook::new().check_installation();
}

fn main() {
    let mut args = args_os();

    match args
        .next()
        .as_deref()
        .map(Path::new)
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
    {
        Some("pre-commit") => pre_commit_hook::run(false),
        _ => {
            if let Some(subcommand) = args.next() {
                match subcommand.to_str() {
                    Some("ci") => {
                        pre_commit_hook::run(true);
                    }
                    Some("pre-commit") => {
                        pre_commit_hook::run(false);
                    }
                    Some("setup-environment") => {
                        setup_environment();
                    }
                    Some("check-environment") => {
                        check_environment();
                    }
                    Some("crawl") => {
                        cargo("crawl", args);
                    }
                    Some("serve") => {
                        cargo("serve", args);
                    }
                    Some("screenshot") => {
                        cargo("screenshot", args);
                    }
                    Some("visual_diff") => {
                        cargo("visual_diff", args);
                    }
                    Some("generate") => {
                        cargo("generate", args);
                    }
                    Some("watch") => {
                        cargo("watch", args);
                    }
                    Some("flatten_yaml") => {
                        assert!(cargo_self("flatten_yaml", args).success());
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
}
