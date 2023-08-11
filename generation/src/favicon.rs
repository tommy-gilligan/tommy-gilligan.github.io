use crate::cache::Cache;
use image::ImageFormat;
use tl::Node::Tag;
use url::Position;
use url::Url;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::from_utf8,
};

pub struct Favicon {
    url: Url,
    cache: Cache,
}

impl Favicon {
    #[must_use]
    pub fn for_url(url: &Url, cache: Cache) -> Self {
        let body_bytes = cache.blocking_get(url).unwrap();
        let body = from_utf8(&body_bytes).unwrap();
        let dom = tl::parse(body, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let mut base_url: Url = url[..Position::BeforePath].parse().unwrap();
        let mut link_query = dom.query_selector(r#"link[rel~="icon"][href]"#).unwrap();

        if let Some(node_handle) = link_query.next() {
            if let Tag(tag) = node_handle.get(parser).unwrap() {
                let favicon_url = url::Url::options()
                    .base_url(Some(&base_url))
                    .parse(
                        tag.attributes()
                            .get("href")
                            .unwrap()
                            .unwrap()
                            .try_as_utf8_str()
                            .unwrap(),
                    )
                    .unwrap();
                Self {
                    url: favicon_url,
                    cache,
                }
            } else {
                base_url.path_segments_mut().unwrap().push("favicon.ico");
                Self {
                    url: base_url,
                    cache,
                }
            }
        } else {
            base_url.path_segments_mut().unwrap().push("favicon.ico");
            Self {
                url: base_url,
                cache,
            }
        }
    }

    fn source(&self) -> Option<Vec<u8>> {
        self.cache.blocking_get(&self.url)
    }

    fn loaded_source(&self) -> Option<image::error::ImageResult<image::DynamicImage>> {
        self.source().map(|source| image::load_from_memory(&source))
    }

    #[must_use]
    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        Hash::hash_slice(self.url.as_str().as_bytes(), &mut hasher);
        hasher.finish()
    }

    pub fn write<W>(&self, mut w: W)
    where
        W: std::io::Write + std::io::Seek,
    {
        if let Some(Ok(image)) = self.loaded_source() {
            image::imageops::resize(&image, 16, 16, image::imageops::FilterType::Nearest)
                .write_to(&mut w, ImageFormat::Ico)
                .unwrap();
        }
    }
}
