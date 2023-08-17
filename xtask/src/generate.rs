use build_time::build_time_local;
use generation::{
    article::Article,
    cache::Cache,
    favicon::Favicon,
    github::Remote,
    layout::{Factory, Layout},
    markdown::Markdown,
    output::Output,
    style::Style,
    view::{Author, Footer, History},
    view::{Link, LinkList},
};
use git2::Repository;
use git_version::git_version;
use rss::ChannelBuilder;
use rss::ItemBuilder;
use std::process::{Child, Command};
use std::{io::Write, path::Path};
use url::Url;

pub fn layout_for_page(factory: &Factory, body: &str, article: &Article) -> String {
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
        title: factory.title,
        language: factory.language,
        style: &factory.style.style(),
        description: &article.description(),
        body,
        page_title: Some(&article.title()),
        footer: &footer,
        author: "",
    }
    .to_string()
}

pub fn layout_for_index(factory: &Factory, body: &str) -> String {
    Layout {
        title: factory.title,
        language: factory.language,
        style: &factory.style.style(),
        description: "",
        body,
        page_title: None,
        footer: "",
        author: "",
    }
    .to_string()
}

pub fn generator() -> String {
    if env!("CARGO_PKG_REPOSITORY").is_empty() {
        format!(
            "{} version: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    } else {
        format!(
            "{} version: {}",
            env!("CARGO_PKG_REPOSITORY"),
            git_version!()
        )
    }
}

pub fn channel_builder(config: &Args) -> ChannelBuilder {
    let mut channel = ChannelBuilder::default();
    channel
        .title(config.title.clone())
        .link(config.base_url.clone())
        //.description(crate::config::description())
        // should only change when content has changed
        .last_build_date(Some(
            build_time_local!("%a, %d %b %Y %H:%M:%S %z").to_string(),
        ))
        .language(config.language.clone())
        .ttl("600".to_string())
        .generator(generator());
    channel
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub base_url: Url,
    #[arg(short, long, default_value = "_site")]
    pub output: String,
    #[arg(short, long, default_value = "cache")]
    pub cache: String,
    #[arg(short, long, default_value = "articles")]
    pub articles: String,
    #[arg(short, long, default_value = "Tommy's Blog")]
    pub title: String,
    #[arg(short, long, default_value = "en-AU")]
    pub language: String,
}

pub fn run() -> Child {
    Command::new("cargo")
        .arg("xtask")
        .arg("generate")
        .arg("--error")
        .spawn()
        .expect("failed to execute process")
}

pub fn generate(config: &Args) {
    let mut channel = &mut channel_builder(config);
    let output = Output::new(&config.output);
    let style = Style::new(Path::new("style.css"));
    let layout_factory = Factory {
        style,
        title: &config.title,
        language: &config.language,
    };
    let mut index_entries: Vec<(String, String)> = Vec::new();
    for article in Article::from_dir(&config.articles).unwrap() {
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
                let cache = Cache::new(&config.cache);
                let furl = url.parse().unwrap();
                let favicon = Favicon::for_url(&furl, cache);
                let file_name = format!("{}.ico", favicon.hash());
                let file = std::fs::File::create(Path::new(&config.output).join(file_name.clone()))
                    .unwrap();
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

        let mut item_builder = ItemBuilder::default();
        let item = item_builder
            .title(article.title())
            .description(article.description())
            .pub_date(
                article
                    .published_at()
                    .format("%a, %d %b %Y %H:%M:%S %z")
                    .to_string(),
            );

        channel = channel.item(item.build());

        let mut output_file = output.page(article.file_stem());
        index_entries.push((
            output
                .page_path(article.file_stem())
                .file_name()
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

    let channel = channel.build();
    let feed = output.feed();
    channel.write_to(feed).unwrap();
}
