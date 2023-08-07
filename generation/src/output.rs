use sitemap::{
    reader::{SiteMapEntity, SiteMapReader},
    structs::{Location, UrlEntry},
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
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        let file = File::create(path.join("sitemap.xml")).unwrap();
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

pub struct Sitemap {
    parser: SiteMapReader<File>,
}

impl Sitemap {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        let file = File::open(path.join("sitemap.xml")).expect("Unable to open file.");
        Self {
            parser: SiteMapReader::new(file),
        }
    }
}

impl Iterator for Sitemap {
    type Item = Url;

    fn next(&mut self) -> Option<Url> {
        for entity in self.parser.by_ref() {
            if let SiteMapEntity::Url(UrlEntry {
                loc: Location::Url(url),
                ..
            }) = entity
            {
                return Some(url);
            }
        }
        None
    }
}

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
    pub fn create_sitemap(&self) -> SitemapBuilder {
        SitemapBuilder::new(self.path.clone())
    }

    #[must_use]
    pub fn open_sitemap(&self) -> Sitemap {
        Sitemap::new(self.path.clone())
    }
}
