use crate::{
    article::Article,
    cache::Cache,
    config::Config,
    favicon,
    layout::{Factory, Layout},
    markdown::Markdown,
    output::Output,
    style::Style,
};

use std::{io::Write, path::Path};

#[must_use]
pub fn layout_for_page(factory: &Factory, body: &str, article: &Article) -> String {
    Layout {
        title: factory.title,
        language: factory.language,
        style: &factory.style.style(),
        description: &article.description(),
        body,
        page_title: Some(&article.title()),
    }
    .to_string()
}

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub fn render(config: &Config) {
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
        m.replace(|node| favicon::decorate_link(&cache, &config.output, node));
        output
            .page(article.file_stem())
            .write_all(layout_for_page(&layout_factory, &m.render(), &article).as_bytes())
            .unwrap();
    }
}
