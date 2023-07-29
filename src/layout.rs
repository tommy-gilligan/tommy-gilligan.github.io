mod markdown_options;
use crate::syntax_highlighting::format_code;

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

pub fn render_code(contents: &str) -> String {
    let mdast = markdown::to_mdast(contents, &markdown_options::MARKDOWN_OPTIONS.parse).unwrap();

    for child in (&mdast.children()).unwrap() {
        match child {
            markdown::mdast::Node::Code(markdown::mdast::Code { value, lang, .. }) => {
                return Layout {
                    content: &format_code(value)
                }
                .to_string();
            },
            _ => ()
        }
    }
    panic!()
}

pub fn render(contents: &str) -> String {
    Layout { content: &markdown::to_html_with_options(contents, &markdown_options::MARKDOWN_OPTIONS).unwrap() }.to_string()
}
