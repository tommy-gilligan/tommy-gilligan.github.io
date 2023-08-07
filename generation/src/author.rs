use url::Url;

pub struct Author {
    pub name: String,
    pub email: String,
}

impl Author {
    pub fn gravatar_url(&self, size: Option<u16>) -> Url {
        let sum = md5::compute(self.email.trim().to_lowercase().as_bytes());
        let mut url: Url = "https://www.gravatar.com/avatar/".parse().unwrap();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .push(&format!("{sum:x}"));
        url.query_pairs_mut()
            .append_pair("s", &(size.unwrap_or(80).to_string()));
        url
    }
}

markup::define! {
    AuthorView(author: Author) {
        figure.author {
            img[src = author.gravatar_url(None).to_string(), alt = &author.name];
            figcaption {
                author.name;
            }
        }
    }
}
