use site_map::{
    reader::{SiteMapEntity, SiteMapReader},
    structs::{Location, UrlEntry},
    writer::SiteMapWriter,
};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use url::Url;

// https://creativecommons.org/licenses/by-sa/2.5/
// Sitemaps.org: Google, Inc., Yahoo, Inc., and Microsoft Corporation
const SITEMAP_XSD: &[u8; 3728] = include_bytes!("sitemap.xsd");

pub struct Sitemap {
    path: PathBuf,
}

impl Sitemap {
    #[must_use]
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf().join(crate::SITEMAP),
        }
    }

    #[must_use]
    pub fn create(self) -> Builder {
        Builder {
            urls: Vec::new(),
            path: self.path,
        }
    }

    #[must_use]
    pub fn open(self) -> Reader {
        let file = File::open(self.path).expect("Unable to open file.");
        Reader {
            parser: SiteMapReader::new(file),
        }
    }
}

pub struct Builder {
    urls: Vec<Url>,
    path: PathBuf,
}

impl Drop for Builder {
    fn drop(&mut self) {
        let file = File::create(&self.path).unwrap();
        let sitemap_writer = SiteMapWriter::new(file);
        let mut url_writer = sitemap_writer
            .start_urlset()
            .expect("Unable to write urlset");

        for url in &self.urls {
            url_writer
                .url(UrlEntry::builder().loc(url.to_string()))
                .expect("Unable to write url");
        }
        url_writer.end().unwrap();

        if crate::xml::validate(
            std::fs::read_to_string(&self.path).unwrap().as_bytes(),
            Some(SITEMAP_XSD),
        ) != crate::xml::MyResult::Ok
        {
            panic!("bad")
        }
    }
}

impl Builder {
    pub fn push(&mut self, url: &Url) {
        self.urls.push(url.clone());
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

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_writer() {
        let url: Url = "http://example.com/mypage".parse().unwrap();
        let dir = tempdir().unwrap();
        let mut sitemap = Sitemap::new(dir.path()).create();
        sitemap.push(&url);
        drop(sitemap);

        let parser = libxml::parser::Parser::default();
        let doc = parser
            .parse_file(dir.path().join(crate::SITEMAP).to_str().unwrap())
            .unwrap();

        let root = doc.get_root_element().unwrap();
        assert_eq!(root.get_name(), "urlset".to_owned());

        let children = root.get_child_elements();
        assert_eq!(
            children
                .clone()
                .into_iter()
                .map(|n| n.get_name())
                .collect::<Vec<String>>(),
            vec!["url".to_owned()]
        );

        let children = children.first().unwrap().get_child_elements();
        assert_eq!(
            children
                .clone()
                .into_iter()
                .map(|n| n.get_name())
                .collect::<Vec<String>>(),
            vec!["loc".to_owned()]
        );

        assert_eq!(
            &children.first().unwrap().get_content(),
            "http://example.com/mypage"
        );
    }
}
