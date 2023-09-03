use std::env::var;
use std::ffi::OsStr;
use std::process::{Child, ExitStatus, Stdio};
use tokio::process::Command;

pub async fn spawn<S: AsRef<OsStr>>(arg: S) -> std::io::Result<ExitStatus> {
    let mut command = Command::new(var("CARGO").unwrap_or("cargo".to_owned()));

    if cfg!(target_os = "windows") {
        command
            .arg("build")
            .arg("--bin")
            .arg(arg)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .await
    } else {
        command
            .arg("build")
            .arg("--bin")
            .arg(arg)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .await
    }
}

pub fn block_spawn<S: AsRef<OsStr>>(command: S) -> std::io::Result<Child> {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
    }
}
