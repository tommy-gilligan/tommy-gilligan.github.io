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

    #[must_use]
    pub fn earliest(&self, a_path: &Path) -> Commit {
        self.commits_for(a_path)
            .into_iter()
            .min_by_key(git2::Commit::time)
            .unwrap()
    }

    #[must_use]
    pub fn latest(&self, a_path: &Path) -> Commit {
        self.commits_for(a_path)
            .into_iter()
            .max_by_key(git2::Commit::time)
            .unwrap()
    }
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
