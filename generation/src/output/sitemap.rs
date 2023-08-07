use site_map::{
    reader::{SiteMapEntity, SiteMapReader},
    structs::{Location, UrlEntry},
    writer::{SiteMapWriter, UrlSetWriter},
};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use url::Url;

pub struct Sitemap {
    path: PathBuf,
}

impl Sitemap {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf().join("sitemap.xml"),
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

pub struct Builder {
    url_writer: UrlSetWriter<File>,
}

impl Builder {
    pub fn push(&mut self, url: &Url) {
        assert_eq!(url.scheme(), "https");
        self.url_writer
            .url(UrlEntry::builder().loc(url.to_string()))
            .expect("Unable to write url");
    }
}

pub struct Reader {
    parser: SiteMapReader<File>,
}

impl Iterator for Reader {
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
