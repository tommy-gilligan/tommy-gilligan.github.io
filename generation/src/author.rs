use url::Url;
use crate::srcset::srcset;

markup::define! {
    AuthorView<F>(name: String, image_url_for: F) where F: Fn(u16) -> Url {
        div.author {
            @name
            br;
            img[
                loading = "lazy",
                fetchpriority = "low",
                decoding = "aync",
                src = image_url_for(120).to_string(),
                srcset = srcset(image_url_for, 120),
                alt = &name
            ];
        }
    }
}
