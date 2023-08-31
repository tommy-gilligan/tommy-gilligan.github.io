use std::collections::BTreeSet;
use std::collections::HashSet;
use std::net::SocketAddr;

use url::Url;

pub struct Crawler {
    to_visit: BTreeSet<Url>,
    visited: HashSet<Url>,
    driver: crate::browser::Browser,
    origin: Option<url::Origin>,
}

fn root_for(local_addr: &std::net::SocketAddr) -> url::Url {
    format!("http://{}:{}", local_addr.ip(), local_addr.port())
        .parse()
        .unwrap()
}

impl Crawler {
    pub async fn new(local_addr: &SocketAddr) -> Self {
        let mut to_visit = BTreeSet::new();
        to_visit.insert(root_for(local_addr));

        Self {
            driver: crate::browser::Browser::new(local_addr).await,
            to_visit,
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
