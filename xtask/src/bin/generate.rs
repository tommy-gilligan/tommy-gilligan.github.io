#![feature(async_closure)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::io::Write;

use generation::{
    cache::Cache,
    favicon::Favicon,
    layout::{Factory, Layout},
    markdown::Markdown,
    output::Output,
    page::Page,
    style::Style,
};

use std::path::Path;

fn layout_for_page(factory: &Factory, page: &str) -> String {
    Layout {
        title: &factory.title,
        language: &factory.language,
        style: &factory.style.style(),
        description: "",
        body: page,
        page_title: None,
        footer: "",
        author: "",
    }
    .to_string()
}

markup::define! {
    Link<'a>(href: &'a str, text: &'a str, favicon: Option<&'a str>) {
        a [href = href] {
            @if favicon.is_some() {
                img [src = favicon.unwrap()];
            }
            @text
        }
    }
}

fn main() {
    let output = Output::new("./_site");
    let style = Style::new(Path::new("style.css"));
    let layout_factory = Factory {
        style,
        title: "My Blog".to_string(),
        language: "en-AU".to_string(),
    };
    for page in Page::from_dir("pages").unwrap() {
        let mut m = Markdown::new(page.contents());
        m.replace(|node| match node {
            markdown::mdast::Node::Link(markdown::mdast::Link { url, children, .. }) => {
                let values = children
                    .iter()
                    .filter_map(|node| match node {
                        markdown::mdast::Node::InlineCode(markdown::mdast::InlineCode {
                            value,
                            ..
                        })
                        | markdown::mdast::Node::Text(markdown::mdast::Text { value, .. }) => {
                            Some(value)
                        }
                        _ => None,
                    })
                    .fold(String::new(), |acc, x| format!("{} {}", acc, x));
                let cache = Cache::new("./cache");
                let furl = url.parse().unwrap();
                let favicon = Favicon::for_url(&furl, cache);
                let file_name = format!("{}.ico", favicon.hash());
                let file =
                    std::fs::File::create(Path::new("_site").join(file_name.clone())).unwrap();
                favicon.write(file);

                Some(
                    Link {
                        href: url,
                        text: &values,
                        favicon: Some(&file_name),
                    }
                    .to_string(),
                )
            }
            _ => None,
        });

        let mut output_file = output.page(page.file_stem());
        output_file
            .write_all(layout_for_page(&layout_factory, &m.render()).as_bytes())
            .unwrap();
    }
}
