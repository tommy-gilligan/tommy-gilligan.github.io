fn combined_title(title: &str, page_title: Option<&str>) -> String {
    if page_title.is_some() {
        format!("{} - {}", page_title.unwrap(), title)
    } else {
        title.to_string()
    }
}

markup::define! {
    Layout<'a>(
        title: &'a str,
        author: &'a str,
        description: &'a str,
        body: &'a str,
        language: &'a str,
        page_title: Option<&'a str>,
        style: &'a str,
    ) {
        @markup::doctype()
        html[lang = language] {
            head {
                title { @combined_title(title, *page_title) }
                meta[charset = "utf-8"];
                meta["http-equiv" = "Content-Security-Policy", content = "default-src 'none'; script-src 'none'; script-src-elem 'self'; script-src-attr 'none'; style-src 'none'; style-src-elem 'unsafe-inline'; style-src-attr 'none'; img-src 'self' data:; font-src 'none'; connect-src 'none'; media-src 'none'; object-src 'none'; child-src 'none'; frame-src 'none'; worker-src 'none'; form-action 'none'; upgrade-insecure-requests; block-all-mixed-content; base-uri 'self'; manifest-src 'self'"];
                meta[name = "author", content = author];
                meta[name = "description", content = description];
                link[rel = "alternate", r#type = "application/rss+xml", href = "pages.xml", title = title];
                style {
                    @markup::raw(style)
                }
            }
            body {
                header {
                    h1 { @title }
                    h2 { @page_title }
                }
                main { @markup::raw(body) }
            }
        }
    }
}
