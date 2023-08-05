#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use git2::Repository;

#[allow(dead_code)]
mod config;

#[tokio::main]
async fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {e}"),
    };
    let tree = repo.find_reference("HEAD").unwrap().peel_to_tree();
    for d in repo
        .diff_tree_to_workdir(Some(&tree.unwrap()), None)
        .unwrap()
        .deltas()
    {
        assert!(
            !d.old_file().path().unwrap().starts_with("screenshots/"),
            "changes in screenshots detected"
        );
    }
}
