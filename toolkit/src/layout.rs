fn combined_title(page_title: Option<&str>) -> String {
    if page_title.is_some() {
        format!("{} - {}", page_title.unwrap(), crate::TITLE)
    } else {
        crate::TITLE.to_string()
    }
}

const CSP: &str = "default-src 'none'; script-src 'self' 'unsafe-inline'; script-src-elem https://gist.github.com 'self' 'unsafe-inline'; script-src-attr 'none'; style-src 'self'; style-src-elem https://github.githubassets.com 'self' 'unsafe-inline'; style-src-attr 'unsafe-inline'; img-src 'self' data: http: https:; font-src 'none'; connect-src 'self'; media-src 'none'; object-src 'none'; child-src 'none'; frame-src 'none'; worker-src 'none'; form-action 'none'; upgrade-insecure-requests; block-all-mixed-content; base-uri 'self'; manifest-src 'self';";

markup::define! {
    Layout<'a>(
        description: &'a str,
        body: &'a str,
        page_title: Option<&'a str>,
    ) {
        @markup::doctype()
        html[lang = crate::locale::language_tag()] {
            head {
                title { @combined_title(*page_title) }
                meta[charset = "utf-8"];
                meta["http-equiv" = "Content-Security-Policy", content = markup::raw(CSP)];
                meta[name = "description", content = description];
                meta[name = "viewport", content = "width=device-width, initial-scale=1"];
                link[rel = "alternate", r#type = "application/rss+xml", href = crate::SITEMAP, title = crate::TITLE];
                link[rel = "icon", href = "data:;base64,iVBORw0KGgo="];
                link[rel = "stylesheet", href = "style.css"];
            }
            body {
                header {
                    h1 { a[href = "/"] { @crate::TITLE } }
                    h2 { @page_title }
                }
                main { @markup::raw(body) }
            }
        }
    }
}
