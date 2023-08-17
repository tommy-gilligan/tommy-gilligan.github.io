use generation::{
    layout::{Layout, Factory},
    view::{Footer, Author, History},
    article::Article,
    github::Remote
};
use git2::Repository;
use url::Url;
use build_time::build_time_local;
use rss::ChannelBuilder;
use git_version::git_version;

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

pub fn layout_for_index(factory: &Factory, body: &str) -> String {
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
