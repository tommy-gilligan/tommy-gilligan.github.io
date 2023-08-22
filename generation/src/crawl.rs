use std::collections::BTreeSet;
use std::collections::HashSet;
use std::net::SocketAddr;
use url::{Position, Url};

pub struct Crawler {
    to_visit: BTreeSet<Url>,
    visited: HashSet<Url>,
    local_request: LocalRequest,
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

pub struct LocalRequest {
    local_addr: SocketAddr,
    client: reqwest::Client,
}

impl LocalRequest {
    #[must_use]
    pub fn new(local_addr: SocketAddr) -> Self {
        Self {
            local_addr,
            client: reqwest::Client::builder().build().unwrap(),
        }
    }

    fn rewrite_for_local_access(&self, url: &Url) -> Url {
        format!(
            "http://{}:{}{}",
            self.local_addr.ip(),
            self.local_addr.port(),
            &url[Position::BeforePath..]
        )
        .parse()
        .unwrap()
    }

    #[must_use]
    pub fn get(&self, url: &Url) -> String {
        futures::executor::block_on(async {
            self.client
                .get(self.rewrite_for_local_access(url))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        })
    }
}

impl Crawler {
    #[must_use]
    pub fn new(local_addr: SocketAddr) -> Self {
        Self {
            local_request: LocalRequest::new(local_addr),
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
                let body = self.local_request.get(&to_visit);

                for url in urls(&body, &to_visit) {
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
