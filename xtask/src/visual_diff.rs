use git2::Repository;
use viuer::{print_from_file, Config};

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args;

pub fn visual_diff(_config: &Args) {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {e}"),
    };
    let tree = repo.find_reference("HEAD").unwrap().peel_to_tree();
    for d in repo
        .diff_tree_to_workdir(Some(&tree.unwrap()), None)
        .unwrap()
        .deltas()
    {
        if d.old_file().path().unwrap().starts_with("screenshots/") {
            let old_conf = Config {
                x: 0,
                y: 0,
                width: Some(80),
                height: Some(25),
                ..Default::default()
            };
            print_from_file(d.old_file().path().unwrap(), &old_conf)
                .expect("Image printing failed.");
            panic!(
                "changes in screenshot {:?} detected",
                d.old_file().path().unwrap()
            );
        }
    }
}
