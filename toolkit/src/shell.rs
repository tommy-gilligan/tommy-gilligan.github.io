use std::process::{Child, Command, Stdio};

#[must_use]
pub fn spawn(command: &str) -> Child {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .expect("failed to execute process")
    }
}
