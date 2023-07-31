mod markdown_options;

// <meta http-equiv="refresh" content="3;url=https://www.mozilla.org" />
markup::define! {
    Layout<'a>(content: &'a str) {
        @markup::doctype()
        html {
            head {
                meta[charset = "utf-8"];
                meta[http_equiv = "X-Frame-Options", content = "DENY"];
                style { @include_str!("layout.css") }
            }
            body {
                header {
                    h1 { "My Blog" }
                }
                main { @markup::raw(content) }
            }
        }
    }
}

pub fn render(contents: &str) -> String {
    Layout {
        content: &markdown::to_html_with_options(contents, &markdown_options::MARKDOWN_OPTIONS)
            .unwrap(),
    }
    .to_string()
}
