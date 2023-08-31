use std::process::Command;

pub fn run() {
    assert!(Command::new("npx")
        .current_dir(crate::SITE)
        .args(["--yes", "prettier", "*.html", "--write"])
        .status()
        .unwrap()
        .success());
}
