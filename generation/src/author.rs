use url::Url;

pub fn srcset<F>(image_url_for: F) -> String where F: Fn(u16) -> Url {
    format!(
        "{} 1x, {} 2x, {} 3x",
        image_url_for(120),
        image_url_for(240),
        image_url_for(360),
    )
}

markup::define! {
    AuthorView<F>(name: String, email: String, image_url_for: F) where F: Fn(u16) -> Url {
        div.author {
            @name
            br;
            img[
                loading = "lazy",
                fetchpriority = "low",
                decoding = "aync",
                src = image_url_for(120).to_string(),
                srcset = srcset(image_url_for),
                alt = &name
            ];
        }
    }
}
