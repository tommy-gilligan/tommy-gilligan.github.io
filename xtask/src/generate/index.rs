use generation::{
    article::Article,
    layout::{Factory, Layout},
    output::Output,
    style::Style,
    view::LinkList,
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

pub fn render(config: &crate::generate::Args) {
    let output = Output::new(&config.output);
    let style = Style::new(Path::new("style.css"));

    let layout_factory = Factory {
        style,
        title: &config.title,
        language: &config.language,
    };
    let index_entries: Vec<(String, String)> = Article::from_dir(&config.articles)
        .unwrap()
        .into_iter()
        .map(|article| {
            (
                output
                    .page_path(article.file_stem())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                article.title(),
            )
        })
        .collect();
    output
        .index()
        .write_all(
            layout_for(
                &layout_factory,
                &LinkList {
                    links: index_entries,
                }
                .to_string(),
            )
            .as_bytes(),
        )
        .unwrap();
}
