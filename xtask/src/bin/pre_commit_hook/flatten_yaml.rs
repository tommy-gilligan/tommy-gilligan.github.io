use git2::{Repository, Status, Tree};
use std::path::Path;

pub fn flatten_yaml(repository: &Repository, _head: &Tree) {
    let ci_yaml = Path::new("ci.yml");
    let target = Path::new(".github/workflows/ci.yml");
    assert!(ci_yaml.exists());
    assert!(target.exists());

    if repository
        .status_file(ci_yaml)
        .unwrap()
        .contains(Status::INDEX_MODIFIED)
    {
        crate::flatten_yaml::check(ci_yaml, target);
    }
}
