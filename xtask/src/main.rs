use std::env::{args_os, var};
use std::ffi::OsStr;

use std::process::{Command, ExitStatus};

mod pre_commit_hook;

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
        "pre-commit" => pre_commit_hook::run(),
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
                    _ => unimplemented!(),
                }
            }
        }
    }
}
