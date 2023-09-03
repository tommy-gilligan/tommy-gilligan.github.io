mod css;
use crate::output::Output;

use std::{
    ffi::OsStr,
    fs::read_dir,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Asset {
    path: PathBuf,
}

impl Asset {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[must_use]
    pub fn file_name(&self) -> &OsStr {
        self.path.file_name().unwrap()
    }

    pub fn from_dir(path_str: &str) -> std::io::Result<Vec<Self>> {
        let path = Path::new(path_str);
        assert!(path.is_dir());
        Ok(read_dir(path)?
            .filter_map(|e| {
                e.map_or(None, |f| match f.file_type() {
                    Ok(file_type) if file_type.is_file() => Some(Self::new(f.path())),
                    _ => None,
                })
            })
            .collect())
    }
}

pub fn watch<W>(watcher: &mut W)
where
    W: notify::Watcher,
{
    watcher
        .watch(Path::new(crate::ASSETS), notify::RecursiveMode::Recursive)
        .unwrap();
}

pub fn copy() {
    for asset in Asset::from_dir(crate::ASSETS).unwrap() {
        match asset.path.extension() {
            Some(extension) if extension == OsStr::new(css::EXTENSION) => {
                Output::asset(asset.file_name())
                    .write_all(&css::transform(&asset.path))
                    .unwrap();
            }
            _ => {
                std::fs::copy(&asset.path, Output::asset_path(asset.file_name())).unwrap();
            }
        }
    }
}
