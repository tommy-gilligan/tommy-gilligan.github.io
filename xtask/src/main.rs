use std::env::{args_os, var};
use std::ffi::OsStr;
use std::process::Command;
use std::process::ExitStatus;

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

fn main() {
    let mut args = args_os().skip(1);

    match args.next().unwrap().to_str().unwrap() {
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
