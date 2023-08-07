use std::process::Command;

pub fn run() {
    Command::new("npx")
        .args(["prettier", "*.html", "--write"])
        .output()
        .expect("failed to execute process");
}
