use crate::{article::Article, layout::Layout, output::Output, view::ArticleList};

use askama::Template;
use std::io::Write;

pub fn render() {
    let articles: Vec<Article> = Article::from_dir(crate::ARTICLES)
        .unwrap()
        .into_iter()
        .collect();
    let layed_out = Layout {
        description: "",
        body: &ArticleList { articles }.to_string(),
        lang: "en-AU",
        sitemap: "sitemap",
        title: crate::TITLE,
        page_title: None,
    }
    .render()
    .unwrap();

    Output::index().write_all(layed_out.as_bytes()).unwrap();
}
