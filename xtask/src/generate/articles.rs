use generation::{
    article::Article,
    cache::Cache,
    github::Remote,
    layout::{Factory, Layout},
    markdown::Markdown,
    output::Output,
    style::Style,
    view::{Author, Footer, History},
};
use git2::Repository;

use std::{io::Write, path::Path};

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

pub fn render(config: &crate::generate::Args) {
    let output = Output::new(&config.output);
    let style = Style::new(Path::new("style.css"));
    let layout_factory = Factory {
        style,
        title: &config.title,
        language: &config.language,
    };
    let cache = Cache::new(&config.cache);
    for article in Article::from_dir(&config.articles).unwrap() {
        let mut m = Markdown::new(article.contents());
        m.replace(|node| generation::favicon::decorate_link(&cache, &config.output, node));
        output
            .page(article.file_stem())
            .write_all(layout_for_page(&layout_factory, &m.render(), &article).as_bytes())
            .unwrap();
    }
}
