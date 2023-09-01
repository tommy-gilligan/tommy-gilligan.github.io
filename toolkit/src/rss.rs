use crate::{article::Article, output::Output};
use build_time::build_time_local;

use git_version::git_version;
use rss::ChannelBuilder;
use rss::ItemBuilder;

#[must_use]
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

#[must_use]
pub fn channel_builder(base_url: &url::Url) -> ChannelBuilder {
    let mut channel = ChannelBuilder::default();
    channel
        .title(crate::TITLE)
        .link(base_url.clone())
        //.description(crate::config::description())
        // should only change when content has changed
        .last_build_date(Some(
            build_time_local!("%a, %d %b %Y %H:%M:%S %z").to_owned(),
        ))
        .language(crate::locale::language_tag())
        .ttl("600".to_owned())
        .generator(generator());
    channel
}

pub fn feed(base_url: &url::Url) {
    let mut channel = &mut channel_builder(base_url);
    for article in Article::from_dir(crate::ARTICLES).unwrap() {
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
    }
    channel.build().write_to(Output::feed()).unwrap();
}
