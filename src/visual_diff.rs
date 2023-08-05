#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use git2::Repository;
use viuer::{print_from_file, Config};

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
        if d.old_file().path().unwrap().starts_with("screenshots/") {
            let old_conf = Config {
                // set offset
                x: 0,
                y: 0,
                // set dimensions
                width: Some(80),
                height: Some(25),
                ..Default::default()
            };
            //let old_conf = Config {
            //    // set offset
            //    x: 20,
            //    y: 4,
            //    // set dimensions
            //    width: Some(80),
            //    height: Some(25),
            //    ..Default::default()
            //};
            print_from_file(d.old_file().path().unwrap(), &old_conf)
                .expect("Image printing failed.");
            // print_from_file(d.new_file().path().unwrap(), &conf).expect("Image printing failed.");
            panic!(
                "changes in screenshot {:?} detected",
                d.old_file().path().unwrap()
            );
        }
    }
}
