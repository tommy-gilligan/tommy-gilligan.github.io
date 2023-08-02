markup::define! {
    Layout<'a>(
        title: &'a str,
        author: &'a str,
        description: &'a str,
        body: &'a str,
        language: &'a str,
        page_title: Option<&'a str>,
    ) {
        @markup::doctype()
        html[lang = language] {
            head {
                title { @title }
                meta[charset = "utf-8"];
                meta["http-equiv" = "Content-Security-Policy", content = "default-src 'none'; script-src 'none'; script-src-elem 'self'; script-src-attr 'none'; style-src 'none'; style-src-elem 'unsafe-inline'; style-src-attr 'none'; img-src 'self' data:; font-src 'none'; connect-src 'none'; media-src 'none'; object-src 'none'; prefetch-src *; child-src 'none'; frame-src 'none'; worker-src 'none'; frame-ancestors 'none'; form-action 'none'; upgrade-insecure-requests; block-all-mixed-content; sandbox allow-same-origin allow-scripts allow-top-navigation; base-uri 'self'; manifest-src 'self'"];
                meta[name = "author", content = author];
                meta[name = "description", content = description];
                link[rel = "alternate", r#type = "application/rss+xml", href = "pages.xml", title = title];
                style { @include_str!("layout.css") }
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
