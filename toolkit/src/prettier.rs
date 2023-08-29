use std::path::Path;
use std::process::Command;

pub fn run(_dir: &Path) {
    assert!(Command::new("npx")
        .current_dir("_site")
        .args(["--yes", "prettier", "*.html", "--write"])
        .status()
        .unwrap()
        .success());
}
