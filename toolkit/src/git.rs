use git2::{Commit, DiffFindOptions, Repository, Revwalk};
use std::collections::HashSet;

use std::fmt::Debug;
use std::fmt::Formatter;
use std::path::Path;
use std::path::PathBuf;

pub struct Git {
    repo: Repository,
}

impl Debug for Git {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl Default for Git {
    fn default() -> Self {
        Self::new()
    }
}

impl Git {
    #[must_use]
    pub fn new() -> Self {
        Self {
            repo: Repository::open_from_env().unwrap(),
        }
    }

    #[must_use]
    pub fn revisions(&self) -> Revwalk {
        let mut tree = self.repo.revwalk().unwrap();
        tree.push_head().unwrap();
        tree
    }

    #[must_use]
    pub fn commits(&self) -> Vec<Commit> {
        self.revisions()
            .map(|oid| self.repo.find_commit(oid.unwrap()).unwrap())
            .collect()
    }

    #[must_use]
    pub fn files(&self, a: &Commit, b: &Commit) -> HashSet<PathBuf> {
        let mut diff_find_options = DiffFindOptions::new();
        diff_find_options.renames(true);
        diff_find_options.copies(true);

        let mut diff = self
            .repo
            .diff_tree_to_tree(Some(&a.tree().unwrap()), Some(&b.tree().unwrap()), None)
            .unwrap();
        diff.find_similar(Some(&mut diff_find_options)).unwrap();

        let paths = diff
            .deltas()
            .filter_map(|delta| delta.new_file().path().map(std::path::Path::to_path_buf))
            .collect::<Vec<PathBuf>>();
        let mut hash: HashSet<PathBuf> = HashSet::new();
        for path in paths {
            hash.insert(path);
        }
        hash
    }

    #[must_use]
    pub fn path_in_diff(
        &self,
        old_commit: &Commit,
        new_commit: &Commit,
        new_path: &Path,
    ) -> Option<PathBuf> {
        let mut diff_find_options = DiffFindOptions::new();
        diff_find_options.renames(true);
        diff_find_options.copies(true);

        let mut diff = self
            .repo
            .diff_tree_to_tree(
                Some(&old_commit.tree().unwrap()),
                Some(&new_commit.tree().unwrap()),
                None,
            )
            .unwrap();
        diff.find_similar(Some(&mut diff_find_options)).unwrap();

        diff.deltas().find_map(|delta| {
            if delta.new_file().path() == Some(new_path) {
                Some(delta.old_file().path().unwrap().to_path_buf())
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn commits_for(&self, a_path: &Path) -> Vec<Commit> {
        let workdir = self.repo.workdir().unwrap();
        let mut a_path = workdir
            .join(a_path)
            .canonicalize()
            .unwrap()
            .strip_prefix(workdir)
            .unwrap()
            .to_path_buf();
        self.commits()
            .windows(2)
            .filter_map(|commit_pair| {
                let new_commit = &commit_pair[0];
                let old_commit = &commit_pair[1];

                self.path_in_diff(old_commit, new_commit, &a_path)
                    .map_or_else(
                        || None,
                        |another_path| {
                            a_path = another_path;
                            Some(new_commit.clone())
                        },
                    )
            })
            .collect()
    }
}

#[test]
fn test_commits_for() {
    let git = Git::new();
    let commits: Vec<String> = git
        .commits_for(Path::new("Cargo.toml"))
        .into_iter()
        .map(|c| c.id().to_string())
        .collect();

    assert_eq!(
        commits,
        [
            "3a1ef750c4c15460e818e32218db87fa4087341e",
            "5e509ef78a5726bd68ccd4dd05d6b51f996dc954",
            "ddd2b1237fe9f3af3ab1f931d3ed18ee51b69700",
            "529bd23e24bab8d6e4f6dfeb717d48b8147da02e",
            "3f8de576d9876e559a012d5713b4fbdaff5bbe6d",
            "565ea5bae6ff22ac9c981aab4daf30f1a7e199b8",
            "addac891faa8ca39a4e7db619f986e0ca0907c2c",
            "05b5514b6e84c166cd0625dc3398a4bd624e47e2",
            "2eeef7d1f865651dcd296cfbbaa19567c653c3e7",
            "1733fa752801869fa19368c7f455d120f11428a8",
            "cb5cee9936edb7def3150d33fd254b9306655ca1",
            "c5ff586b7d49fee4cbcd59ee86c93ef59169b596",
            "7f63641e7d0068cb640ca21b4acc88b8eafe3091",
            "1037305faff4424e07bf51f6da636e6642069c7b",
            "4c4912535a77384fc26ec9f9f63a6326a4926072",
            "8c7fcc066a7e1e7af672b316faf887fb94d49164",
            "a86914d7257849dc487d423448aa931ed8087339",
            "5c5541379278ebe00a3fd3d8ad850aea79efa7e4",
            "49412a598a5f86d45d81223f6e5945e36979fd82",
            "4b0b6251995b33644c3cb5eee500209c3715aeb3",
            "32140412262f601ce40ee9c674deffd069ff92a4",
            "1268768e8853c0549edd6387f7a8088203df836a",
            "075801250cb1645e8b32fe520ad5702405e84a90",
            "bc7547d51b60ad62d269d745fb66e77e3ff9daec",
            "da1a693cd5caf24e05fdca580f2a913fa7b7d661",
            "8960e1117c773a3d65a4f2f269dcc68602de9a83",
            "a08b3fda21159aca5c218e1bdc22e7e85fcf3f69",
            "56431abc7254b00e3d0065cb5df5a9a735065a04",
            "f78ecb749dffdce6d2c56029ed68f91e7ace2dcb",
            "186ed6f1e363f65fda43db271024b1004e5033e8",
            "aed8ed84f5fb734acdd90b67a57d353f8d412dff",
            "cf929cffc041e8574c0c33e599ac68e93f7fe58c"
        ]
    );
}

#[test]
fn test_commits_for_rename() {
    let git = Git::new();
    let commits: Vec<String> = git
        .commits_for(Path::new("articles/git-config.md"))
        .into_iter()
        .map(|c| c.id().to_string())
        .collect();

    assert_eq!(
        commits,
        [
            "22ee7e21109daea9b8707ab4877ce6209959bf11",
            "6df55018c4c0a69631520eb42133a1d268bd0541",
            "3e1cb1176af3b59580ede4c8377cdd6c0d6a6443",
            "627f6b21d6b8584f98ca4ea5c8515817504387f0",
            "9f40faa890eee10b6a7349690b920f0ab80d3075",
            "574839031a1fa9503006abebccb0006b08a09327",
            "ebfed3239064af1db9fade22f145ad285a84aefb",
            "2e1077c20bc0179e08bc5d0b44954e9b11f76622",
            "94ceef25a1cb50bb206acb3e25929c181bc65caf"
        ]
    );
}
