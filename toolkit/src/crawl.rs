use std::collections::BTreeSet;
use std::collections::HashSet;

use url::Url;

pub struct Crawler {
    to_visit: BTreeSet<Url>,
    visited: HashSet<Url>,
    driver: crate::chrome_driver::ChromeDriver,
    origin: Option<url::Origin>,
}

#[must_use]
pub fn urls(page: &str, base_url: &Url) -> HashSet<url::Url> {
    let dom = tl::parse(page, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    dom.query_selector(r#"a[href]"#)
        .unwrap()
        .filter_map(|node_handle| match node_handle.get(parser) {
            Some(tl::Node::Tag(tag)) => {
                let mut url = url::Url::options()
                    .base_url(Some(base_url))
                    .parse(
                        tag.attributes()
                            .get("href")
                            .unwrap()
                            .unwrap()
                            .try_as_utf8_str()
                            .unwrap(),
                    )
                    .unwrap();
                url.set_fragment(None);
                url.set_query(None);
                Some(url)
            }
            _ => None,
        })
        .collect()
}

impl Crawler {
    #[must_use]
    pub fn new(driver: crate::chrome_driver::ChromeDriver) -> Self {
        Self {
            driver,
            to_visit: BTreeSet::new(),
            visited: HashSet::new(),
            origin: None,
        }
    }

    pub fn push(&mut self, url: Url) {
        self.to_visit.insert(url);
    }
}

impl Iterator for Crawler {
    type Item = Url;

    fn next(&mut self) -> Option<Url> {
        loop {
            if let Some(mut to_visit) = self.to_visit.pop_first() {
                to_visit.set_fragment(None);
                to_visit.set_query(None);

                match &self.origin {
                    None => {
                        self.origin = Some(to_visit.origin());
                    }
                    Some(origin) => {
                        if *origin != to_visit.origin() {
                            continue;
                        }
                    }
                }

                let inserted = self.visited.insert(to_visit.clone());
                for url in futures::executor::block_on(async {
                    self.driver.goto(&to_visit).await;
                    self.driver.links().await
                }) {
                    println!("{url}");
                    if !self.visited.contains(&url) {
                        self.to_visit.insert(url);
                    }
                }

                if inserted {
                    return Some(to_visit);
                }
            } else {
                return None;
            }
        }
    }
}
