use git2::{Repository, Status};
use std::path::Path;

pub fn run(force: bool) {
    let ci_yaml = Path::new("ci.yml");
    let target = Path::new(".github/workflows/ci.yml");
    assert!(ci_yaml.exists());
    assert!(target.exists());

    let repository = Repository::open_from_env().unwrap();
    let _head = repository.head().unwrap().peel_to_tree().unwrap();

    if repository
        .status_file(ci_yaml)
        .unwrap()
        .contains(Status::INDEX_MODIFIED)
        || force
    {
        if repository
            .status_file(target)
            .unwrap()
            .contains(Status::WT_MODIFIED)
        {
            eprintln!("unstaged changes to target {:?}", target);
            std::process::exit(1);
        }

        crate::flatten_yaml::check(ci_yaml, target);
    }
}
