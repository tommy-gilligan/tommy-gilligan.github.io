use git2::Repository;
use std::env::{args_os, var};
use std::ffi::OsStr;
use std::process::{Command, ExitStatus};

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
    Command::new(var("CARGO").unwrap_or("cargo".to_owned()))
        .arg("run")
        .arg("--package")
        .arg(package)
        .arg("--")
        .args(args)
        .current_dir(git_directory())
        .status()
        .unwrap()
}

fn cargo_self<I, S>(package: &str, args: I) -> ExitStatus
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(var("CARGO").unwrap_or("cargo".to_owned()))
        .arg("run")
        .arg("--bin")
        .arg(package)
        .arg("--")
        .args(args)
        .current_dir(git_directory())
        .status()
        .unwrap()
}

fn setup_environment() {
    pre_commit_hook::install();
}

fn main() {
    let mut args = args_os();
    let binding = args.next().unwrap();
    let name = binding.to_str().unwrap();
    match name {
        ".git/hooks/pre-commit" => pre_commit_hook::run(),
        _ => {
            setup_environment();

            if let Some(subcommand) = args.next() {
                match subcommand.to_str().unwrap() {
                    "crawl" => {
                        cargo("crawl", args);
                    }
                    "serve" => {
                        cargo("serve", args);
                    }
                    "screenshot" => {
                        cargo("screenshot", args);
                    }
                    "visual_diff" => {
                        cargo("visual_diff", args);
                    }
                    "generate" => {
                        cargo("generate", args);
                    }
                    "watch" => {
                        cargo("watch", args);
                    }
                    "flattenyaml" => {
                        assert!(cargo_self("flattenyaml", args).success());
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
}
