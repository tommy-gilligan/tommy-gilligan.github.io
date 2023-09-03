use std::env::var;
use std::ffi::OsStr;
use std::process::{Child, Command, Stdio};

pub fn spawn<S: AsRef<OsStr>>(arg: S) -> std::io::Result<Child> {
    let mut command = Command::new(var("CARGO").unwrap_or("cargo".to_owned()));

    if cfg!(target_os = "windows") {
        command
            .arg("build")
            .arg("--bin")
            .arg(arg)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
    } else {
        command
            .arg("build")
            .arg("--bin")
            .arg(arg)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
    }
}

pub fn block_spawn<S: AsRef<OsStr>>(command: S) -> std::io::Result<Child> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
    }
}
