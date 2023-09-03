use std::ffi::OsStr;
use std::process::{Child, Command, Stdio};

pub fn replacing_spawn<I, B, S: AsRef<OsStr> + std::fmt::Debug>(
    arg: S,
    args: I,
    existing_child: &mut Option<Child>,
) where
    I: IntoIterator<Item = B>,
    B: std::convert::AsRef<std::ffi::OsStr>,
{
    if let Some(ref mut child) = existing_child {
        if child.try_wait().unwrap().is_none() {
            eprintln!("Killing unfinished {arg:?}");
            child.kill().unwrap();
            child.wait().unwrap();
        }
    }

    eprintln!("Starting {arg:?}");
    *existing_child = Some(
        Command::new(arg)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap(),
    );
}
