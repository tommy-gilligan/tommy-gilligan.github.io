fn combined_title(title: &str, page_title: Option<&str>) -> String {
    if page_title.is_some() {
        format!("{} - {}", page_title.unwrap(), title)
    } else {
        title.to_string()
    }
}

const CSP: &str = "default-src 'none'; script-src 'none'; script-src-elem 'self'; script-src-attr 'none'; style-src 'none'; style-src-elem 'unsafe-inline'; style-src-attr 'none'; img-src 'self' data: http: https:; font-src 'none'; connect-src 'none'; media-src 'none'; object-src 'none'; child-src 'none'; frame-src 'none'; worker-src 'none'; form-action 'none'; upgrade-insecure-requests; block-all-mixed-content; base-uri 'self'; manifest-src 'self'";

markup::define! {
    Layout<'a>(
        title: &'a str,
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
                meta["http-equiv" = "Content-Security-Policy", content = markup::raw(CSP)];
                meta[name = "description", content = description];
                meta[name = "viewport", content = "width=device-width, initial-scale=1, interactive-widget=overlays-content"];
                link[rel = "alternate", r#type = "application/rss+xml", href = "pages.xml", title = title];
                link[rel = "icon", href = "data:;base64,iVBORw0KGgo="];
                style {
                    @markup::raw(style)
                }
            }
            body {
                header {
                    h1 { a[href = "/"] { @title } }
                    h2 { @page_title }
                }
                main { @markup::raw(body) }
            }
        }
    }
}

use crate::style::Style;

#[derive(Clone)]
pub struct Factory<'a> {
    pub title: &'a str,
    pub language: &'a str,
    pub style: Style,
}
