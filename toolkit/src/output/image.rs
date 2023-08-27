use std::{
    fs::File,
    path::{Path, PathBuf},
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher
};
use url::Url;

impl Image {
    pub fn new(path: &Path, hash: u64) -> Self {
        Self {
            path: path.to_path_buf().join(format!("{}", hash)),
        }
    }

    pub fn create(self) -> Builder {
        let file = File::create(self.path).unwrap();
        let sitemap_writer = SiteMapWriter::new(file);
        let url_writer = sitemap_writer
            .start_urlset()
            .expect("Unable to write urlset");

        Builder { url_writer }
    }

    pub fn open(self) -> Reader {
        let file = File::open(self.path).expect("Unable to open file.");
        Reader {
            parser: SiteMapReader::new(file),
        }
    }
}
