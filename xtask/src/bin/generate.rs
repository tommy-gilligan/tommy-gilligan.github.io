#![feature(async_closure)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use git2::Repository;
use std::io::Write;

use generation::{
    author::AuthorView,
    cache::Cache,
    favicon::Favicon,
    github::Remote,
    layout::{Factory, Layout},
    markdown::Markdown,
    output::Output,
    page::Page,
    style::Style,
};

use std::path::Path;

markup::define! {
    FooterView(author: String, revisions: String) {
        @markup::raw(author)
        @markup::raw(revisions)
    }
}

fn layout_for_page(factory: &Factory, body: &str, page: &Page) -> String {
    let commits = page.history();
    let revisions = generation::history::History { commits }.to_string();
    let repo = Repository::open(".").unwrap();
    let github_user = (&repo.remotes().unwrap())
        .into_iter()
        .find_map(|remote_name| {
            match Remote::try_from(repo.find_remote(remote_name.unwrap()).unwrap()) {
                Ok(remote) => Some(remote.user()),
                _ => None,
            }
        })
        .unwrap();

    let author = AuthorView {
        name: "Tommy Gilligan".to_string(),
        image_url_for: |size| github_user.avatar(size),
    }
    .to_string();

    let footer = FooterView { author, revisions }.to_string();
    Layout {
        title: &factory.title,
        language: &factory.language,
        style: &factory.style.style(),
        description: &page.description(),
        body,
        page_title: Some(&page.title()),
        footer: &footer,
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
    for page in Page::from_dir("./pages").unwrap() {
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
            .write_all(layout_for_page(&layout_factory, &m.render(), &page).as_bytes())
            .unwrap();
    }
}
