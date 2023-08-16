use md5::compute;
use url::Url;

const GRAVATAR_URL: &str = "https://www.gravatar.com/avatar/";

pub struct User(String);

impl User {
    pub const fn new(email: String) -> Self {
        Self(email)
    }

    pub fn avatar(&self, size: u16) -> Url {
        let sum = compute(self.0.trim().to_lowercase().as_bytes());
        let mut url: Url = GRAVATAR_URL.parse().unwrap();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .push(&format!("{sum:x}"));
        url.query_pairs_mut().append_pair("s", &(size.to_string()));
        url
    }
}
