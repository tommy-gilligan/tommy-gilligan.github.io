use crate::{article::Article, layout::Layout, output::Output, view::ArticleList};

use std::io::Write;

pub fn render() {
    let articles: Vec<Article> = Article::from_dir(crate::ARTICLES)
        .unwrap()
        .into_iter()
        .collect();
    let layed_out = Layout {
        description: "",
        body: &ArticleList { articles }.to_string(),
        page_title: None,
    }
    .to_string();

    Output::index().write_all(layed_out.as_bytes()).unwrap();
}
