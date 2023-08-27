use git2::Repository;
use toolkit::{
    article::Article,
    cache::Cache,
    github::Remote,
    layout::{Factory, Layout},
    markdown::Markdown,
    output::Output,
    style::Style,
    view::{Author, Footer, History},
};

use std::{io::Write, path::Path};

pub fn layout_for_page(factory: &Factory, body: &str, article: &Article) -> String {
    let repo = Repository::open_from_env().unwrap();
    let github = (&repo.remotes().unwrap())
        .into_iter()
        .find_map(|remote_name| {
            return Remote::try_from(repo.find_remote(remote_name.unwrap()).unwrap()).ok();
        })
        .unwrap();
    let commits = article.truncated_history();
    let revisions = History {
        remote: &github,
        commits,
    }
    .to_string();
    let github_user = github.user();

    let author = Author {
        name: "Tommy Gilligan".to_owned(),
        image_url_for: |size| github_user.avatar(size),
        social_links: vec![
            ("Github".to_owned(), "https://example.com".parse().unwrap()),
            (
                "Mastodon".to_owned(),
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

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub fn render(config: &crate::Config) {
    let output = Output::new(&config.output);
    let style = Style::new(Path::new("style.css"));
    let layout_factory = Factory {
        style,
        title: &config.title,
        language: &config.language,
    };
    let client = reqwest::Client::builder()
        .timeout(core::time::Duration::new(10, 0))
        .user_agent(USER_AGENT)
        .http1_title_case_headers()
        .build()
        .unwrap();
    let cache = Cache::new(&config.cache, client);
    for article in Article::from_dir(&config.articles).unwrap() {
        let mut m = Markdown::new(article.contents());
        m.replace(|node| toolkit::favicon::decorate_link(&cache, &config.output, node));
        output
            .page(article.file_stem())
            .write_all(layout_for_page(&layout_factory, &m.render(), &article).as_bytes())
            .unwrap();
    }
}
