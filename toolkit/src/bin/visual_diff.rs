use git2::Repository;
use viuer::print_from_file;

#[tokio::main]
async fn main() {
    let repo = Repository::open_from_env().unwrap();
    let tree = repo.find_reference("HEAD").unwrap().peel_to_tree();
    for d in repo
        .diff_tree_to_workdir(Some(&tree.unwrap()), None)
        .unwrap()
        .deltas()
    {
        if d.old_file().path().unwrap().starts_with("screenshots") {
            let old_conf = viuer::Config {
                x: 0,
                y: 0,
                width: Some(80),
                height: Some(25),
                ..Default::default()
            };
            print_from_file(d.old_file().path().unwrap(), &old_conf).unwrap();
            panic!(
                "changes in screenshot {:?} detected",
                d.old_file().path().unwrap()
            );
        }
    }
}
