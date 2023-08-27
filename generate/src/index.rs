use toolkit::{
    article::Article,
    layout::{Factory, Layout},
    output::Output,
    style::Style,
    view::ArticleList,
};

use std::{io::Write, path::Path};

pub fn layout_for(factory: &Factory, body: &str) -> String {
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

pub fn render(config: &crate::Config) {
    let output = Output::new(&config.output);
    let style = Style::new(Path::new("style.css"));

    let layout_factory = Factory {
        style,
        title: &config.title,
        language: &config.language,
    };
    let articles: Vec<Article> = Article::from_dir(&config.articles)
        .unwrap()
        .into_iter()
        .collect();
    output
        .index()
        .write_all(layout_for(&layout_factory, &ArticleList { articles }.to_string()).as_bytes())
        .unwrap();
}
