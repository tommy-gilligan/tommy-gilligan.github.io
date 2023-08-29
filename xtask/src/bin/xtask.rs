use git2::Repository;
use std::env::{args_os, var, ArgsOs};
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

struct Task<F: Fn(ArgsOs) + 'static + ?Sized> {
    name: &'static str,
    function: &'static F,
}

const TASKS: [Task<dyn Fn(ArgsOs) + 'static>; 6] = [
    Task {
        name: "check-environment",
        function: &check_environment,
    },
    Task {
        name: "ci",
        function: &ci,
    },
    Task {
        name: "flatten-yaml",
        function: &flatten_yaml,
    },
    Task {
        name: "pre-commit",
        function: &pre_commit,
    },
    Task {
        name: "setup-environment",
        function: &setup_environment,
    },
    Task {
        name: "--help",
        function: &print_help,
    },
];

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

fn ci(_: ArgsOs) {
    if [build(), clippy(), test(), pre_commit_hook::run(true)]
        .iter()
        .any(|t| !t)
    {
        std::process::exit(1);
    }
}

fn setup_environment(_: ArgsOs) {}

fn check_environment(_: ArgsOs) {
    pre_commit_hook::PreCommitHook::new().check_installation();
}

fn print_help(_: ArgsOs) {
    eprintln!("cargo xtask <task name> [options]");
    eprintln!("Available tasks:");
    for task in TASKS {
        eprintln!("{}", task.name);
    }
}

fn pre_commit(_: ArgsOs) {
    pre_commit_hook::run(false);
}

fn flatten_yaml(args: ArgsOs) {
    assert!(cargo_self("flatten_yaml", args).success());
}

pub fn clippy() -> bool {
    eprintln!("clippy()");
    let mut command = Command::new(var("CARGO").unwrap_or("cargo".to_owned()));
    command
        .arg("clippy")
        .arg("--all-targets")
        .arg("--")
        .arg("--deny")
        .arg("warnings")
        .current_dir(git_directory());
    command.status().unwrap().success()
}

pub fn test() -> bool {
    eprintln!("test()");
    let mut command = Command::new(var("CARGO").unwrap_or("cargo".to_owned()));
    command
        .arg("test")
        .arg("--no-fail-fast")
        .current_dir(git_directory());
    command.status().unwrap().success()
}

pub fn build() -> bool {
    eprintln!("build()");
    let mut command = Command::new(var("CARGO").unwrap_or("cargo".to_owned()));
    command
        .arg("build")
        .arg("--all-targets")
        .current_dir(git_directory());
    command.status().unwrap().success()
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
        Some("pre-commit") => {
            if !pre_commit_hook::run(false) {
                std::process::exit(1);
            }
        }
        _ => {
            if let Some(subcommand) = args.next() {
                if let Some(task) = TASKS
                    .iter()
                    .find(|task| Some(task.name) == subcommand.to_str())
                {
                    (task.function)(args);
                } else {
                    print_help(args);
                    std::process::exit(1);
                }
            } else {
                print_help(args)
            }
        }
    }
}
