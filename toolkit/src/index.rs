use crate::{article::Article, layout::Layout, output::Output, view::ArticleList};

use std::io::Write;

fn layout_for(body: &str) -> String {
    Layout {
        title: crate::TITLE,
        language: &crate::locale::language(),
        description: "",
        body,
        page_title: None,
    }
    .to_string()
}

pub fn render() {
    let articles: Vec<Article> = Article::from_dir(crate::ARTICLES)
        .unwrap()
        .into_iter()
        .collect();
    Output::index()
        .write_all(layout_for(&ArticleList { articles }.to_string()).as_bytes())
        .unwrap();
}
