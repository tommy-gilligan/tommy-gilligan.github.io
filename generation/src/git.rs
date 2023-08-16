use git2::{Commit, Repository, Revwalk, DiffFindOptions};
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
            repo: Repository::open(".").unwrap(),
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
    pub fn commits_for(&self, a_path: &Path) -> Vec<Commit> {
        let canonical = a_path.canonicalize().unwrap();
        self.commits()
            .windows(2)
            .filter_map(|commit_pair| {
                if self
                    .files(&commit_pair[0], &commit_pair[1])
                    .into_iter()
                    .any(|path_buf| {
                        path_buf
                            .as_path()
                            .canonicalize()
                            .map_or(false, |other_can| other_can == canonical)
                    })
                {
                    Some(commit_pair[0].clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
