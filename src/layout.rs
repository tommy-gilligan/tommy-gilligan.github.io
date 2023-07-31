mod markdown_options;

markup::define! {
    Layout<'a>(content: &'a str) {
        @markup::doctype()
        html {
            head {
                meta[charset = "utf-8"];
                style { @include_str!("layout.css") }
            }
            body {
                #main { @markup::raw(content) }
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
