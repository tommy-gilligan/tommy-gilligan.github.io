use std::process::{Child, ExitStatus, Stdio};

use tokio::process::Command;

#[must_use]
pub async fn spawn(command: &str) -> std::io::Result<ExitStatus> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .await
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .await
    }
}

pub fn block_spawn(command: &str) -> std::io::Result<Child> {
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
