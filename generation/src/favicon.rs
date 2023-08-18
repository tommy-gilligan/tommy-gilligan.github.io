use crate::cache::Cache;
use image::ImageFormat;
use markdown::mdast::{InlineCode, Link, Node, Text};
use tl::Node::Tag;
use url::{Position, Url};

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
    str::from_utf8,
};

pub struct Favicon<'a> {
    url: Url,
    cache: &'a Cache,
}

impl<'a> Favicon<'a> {
    #[must_use]
    pub fn for_url(url: &Url, cache: &'a Cache) -> Self {
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

#[must_use]
pub fn decorate_link(cache: &Cache, output: &str, node: &Node) -> Option<String> {
    if let Node::Link(Link { url, children, .. }) = node {
        let values = children
            .iter()
            .filter_map(|node| match node {
                Node::InlineCode(InlineCode { value, .. }) | Node::Text(Text { value, .. }) => {
                    Some(value)
                }
                _ => None,
            })
            .fold(String::new(), |acc, x| format!("{acc} {x}"));
        let favicon = Favicon::for_url(&url.parse().unwrap(), cache);
        let file_name = format!("{}.ico", favicon.hash());
        let file = std::fs::File::create(Path::new(&output).join(file_name.clone())).unwrap();
        favicon.write(file);

        Some(
            crate::view::Link {
                href: url,
                text: &values,
                favicon: Some(&file_name),
            }
            .to_string(),
        )
    } else {
        None
    }
}
