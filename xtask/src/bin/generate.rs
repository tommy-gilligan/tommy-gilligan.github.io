#![feature(async_closure)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use git2::Repository;
use std::io::Write;

use generation::{
    cache::Cache,
    favicon::Favicon,
    github::Remote,
    layout::{Factory, Layout},
    markdown::Markdown,
    output::Output,
    article::Article,
    style::Style,
    view::{Author, Link, Footer, History},
};

use std::path::Path;

fn layout_for_page(factory: &Factory, body: &str, article: &Article) -> String {
    let repo = Repository::open(".").unwrap();
    let github = (&repo.remotes().unwrap())
        .into_iter()
        .find_map(|remote_name| {
            match Remote::try_from(repo.find_remote(remote_name.unwrap()).unwrap()) {
                Ok(remote) => Some(remote),
                _ => None,
            }
        })
        .unwrap();
    let commits = article.history();
    let revisions = History { remote: &github, commits }.to_string();
    let github_user = github.user();

    let author = Author {
        name: "Tommy Gilligan".to_string(),
        image_url_for: |size| github_user.avatar(size),
    }
    .to_string();

    let footer = Footer { author, revisions }.to_string();
    Layout {
        title: &factory.title,
        language: &factory.language,
        style: &factory.style.style(),
        description: &article.description(),
        body,
        page_title: Some(&article.title()),
        footer: &footer,
        author: "",
    }
    .to_string()
}

fn main() {
    let output = Output::new("./_site");
    let style = Style::new(Path::new("style.css"));
    let layout_factory = Factory {
        style,
        title: "My Blog".to_string(),
        language: "en-AU".to_string(),
    };
    for article in Article::from_dir("./articles").unwrap() {
        let mut m = Markdown::new(article.contents());
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

        let mut output_file = output.page(article.file_stem());
        output_file
            .write_all(layout_for_page(&layout_factory, &m.render(), &article).as_bytes())
            .unwrap();
    }
}
