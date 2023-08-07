use sitemap::{
    structs::UrlEntry,
    writer::{SiteMapWriter, UrlSetWriter},
};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use url::Url;

pub struct SitemapBuilder {
    url_writer: UrlSetWriter<File>,
}

impl SitemapBuilder {
    pub fn new(path: PathBuf) -> Self {
        let file = File::create(path.clone().join("sitemap.xml")).unwrap();
        let sitemap_writer = SiteMapWriter::new(file);
        let url_writer = sitemap_writer
            .start_urlset()
            .expect("Unable to write urlset");

        Self { url_writer }
    }

    pub fn push(&mut self, url: Url) {
        assert_eq!(url.scheme(), "https");
        self.url_writer
            .url(UrlEntry::builder().loc(url.to_string()))
            .expect("Unable to write url");
    }
}

pub struct Output {
    path: PathBuf,
}

impl Output {
    pub fn new(path: &str) -> Self {
        Self {
            path: Path::new(path).to_path_buf(),
        }
    }

    pub fn create_sitemap(&self) -> SitemapBuilder {
        SitemapBuilder::new(self.path.clone())
    }
}
