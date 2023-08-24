use std::env::{current_exe, args_os, var};
use std::ffi::OsStr;
use std::process::{Command, ExitStatus};
use std::fs::{hard_link, metadata};
use std::env::consts::EXE_EXTENSION;

mod pre_commit;

fn cargo<I, S>(package: &str, args: I) -> ExitStatus
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(var("CARGO").unwrap_or("cargo".to_owned()))
        .arg("run")
        .arg("--package")
        .arg(package)
        .args(args)
        .status()
        .unwrap()
}

fn install_hook() {
    let target = git2::Repository::open_from_env()
        .unwrap()
        .workdir()
        .unwrap()
        .join(".git")
        .join("hooks")
        .join("pre-commit")
        .with_extension(EXE_EXTENSION);
    let source = current_exe().unwrap();

    if !target.exists() {
        hard_link(source, &target).unwrap()
    }
}

fn main() {
    let mut args = args_os();
    let binding = args.next().unwrap();
    let name = binding.to_str().unwrap();
    match name {
        "pre-commit" => {
            pre_commit::run()
        }
        _ => {
            install_hook();

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
                    _ => unimplemented!(),
                }
            }
        }
    }
}
