mod sitemap;
use sitemap::Sitemap;
use std::{
    ffi::OsStr,
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
};

pub struct Output {
    path: PathBuf,
}

impl Output {
    #[must_use]
    pub fn new(path: &str) -> Self {
        create_dir_all(path).unwrap();
        Self {
            path: Path::new(path).to_path_buf(),
        }
    }

    #[must_use]
    pub fn sitemap(&self) -> Sitemap {
        Sitemap::new(&self.path)
    }

    #[must_use]
    pub fn page(&self, file_stem: &OsStr) -> File {
        File::create(self.page_path(file_stem)).unwrap()
    }

    #[must_use]
    pub fn page_path(&self, file_stem: &OsStr) -> PathBuf {
        self.path.clone().join(file_stem).with_extension("html")
    }

    #[must_use]
    pub fn index(&self) -> File {
        File::create(self.path.clone().join("index.html")).unwrap()
    }

    #[must_use]
    pub fn feed(&self) -> File {
        File::create(self.path.clone().join("feed.xml")).unwrap()
    }
}
