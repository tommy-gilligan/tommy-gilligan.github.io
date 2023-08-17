#![feature(async_closure)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use git2::Repository;
use std::io::Write;

use generation::{
    article::Article,
    cache::Cache,
    favicon::Favicon,
    github::Remote,
    layout::{Factory, Layout},
    markdown::Markdown,
    output::Output,
    style::Style,
    view::{Author, Footer, History, Link, LinkList},
};

use std::path::Path;

fn layout_for_page(factory: &Factory, body: &str, article: &Article) -> String {
    let repo = Repository::open_from_env().unwrap();
    let github = (&repo.remotes().unwrap())
        .into_iter()
        .find_map(|remote_name| {
            Remote::try_from(repo.find_remote(remote_name.unwrap()).unwrap()).ok()
        })
        .unwrap();
    let commits = article.history();
    let revisions = History {
        remote: &github,
        commits,
    }
    .to_string();
    let github_user = github.user();

    let author = Author {
        name: "Tommy Gilligan".to_string(),
        image_url_for: |size| github_user.avatar(size),
        social_links: vec![
            ("Github".to_string(), "https://example.com".parse().unwrap()),
            (
                "Mastodon".to_string(),
                "https://example.com".parse().unwrap(),
            ),
        ],
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

fn layout_for_index(factory: &Factory, body: &str) -> String {
    Layout {
        title: &factory.title,
        language: &factory.language,
        style: &factory.style.style(),
        description: "",
        body,
        page_title: None,
        footer: "",
        author: "",
    }
    .to_string()
}

fn main() {
    let output = Output::new("./_site");
    let style = Style::new(Path::new("style.css"));
    let layout_factory = Factory {
        style,
        title: "Tommy's Blog".to_string(),
        language: "en-AU".to_string(),
    };
    let mut index_entries: Vec<(String, String)> = Vec::new();
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
                    .fold(String::new(), |acc, x| format!("{acc} {x}"));
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
        index_entries.push((
            output
                .page_path(article.file_stem())
                .strip_prefix("./_site/")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            article.title(),
        ));

        output_file
            .write_all(layout_for_page(&layout_factory, &m.render(), &article).as_bytes())
            .unwrap();
    }
    let link_list = LinkList {
        links: index_entries,
    }
    .to_string();
    let mut index_file = output.index();
    index_file
        .write_all(layout_for_index(&layout_factory, &link_list).as_bytes())
        .unwrap();
}
