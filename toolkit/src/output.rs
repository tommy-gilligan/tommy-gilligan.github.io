use crate::sitemap::Sitemap;
use std::fs::create_dir_all;
use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

pub struct Output {}

impl Output {
    #[must_use]
    pub fn asset(file_name: &OsStr) -> File {
        File::create(Self::asset_path(file_name)).unwrap()
    }

    #[must_use]
    pub fn asset_path(file_name: &OsStr) -> PathBuf {
        Path::new(crate::SITE).join(file_name)
    }

    #[must_use]
    pub fn page(file_stem: &OsStr) -> File {
        File::create(Self::page_path(file_stem)).unwrap()
    }

    #[must_use]
    pub fn page_path(file_stem: &OsStr) -> PathBuf {
        Path::new(crate::SITE)
            .join(file_stem)
            .with_extension("html")
    }

    #[must_use]
    pub fn index() -> File {
        File::create(Path::new(crate::SITE).join("index.html")).unwrap()
    }

    #[must_use]
    pub fn sitemap() -> Sitemap {
        Sitemap::new(Path::new(crate::SITE))
    }
}
