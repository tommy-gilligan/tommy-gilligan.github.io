use std::path::{Path, PathBuf};
mod sitemap;
use sitemap::Sitemap;

pub struct Output {
    path: PathBuf,
}

impl Output {
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            path: Path::new(path).to_path_buf(),
        }
    }

    #[must_use]
    pub fn sitemap(&self) -> Sitemap {
        Sitemap::new(&self.path)
    }
}
