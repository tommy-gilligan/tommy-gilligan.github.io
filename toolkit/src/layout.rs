fn combined_title(page_title: Option<&str>) -> String {
    if page_title.is_some() {
        format!("{} - {}", page_title.unwrap(), crate::TITLE)
    } else {
        crate::TITLE.to_string()
    }
}

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
