use build_time::build_time_local;
use generation::{article::Article, output::Output};

use git_version::git_version;
use rss::ChannelBuilder;
use rss::ItemBuilder;

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

pub fn channel_builder(config: &crate::Config) -> ChannelBuilder {
    let mut channel = ChannelBuilder::default();
    channel
        .title(config.title.clone())
        .link(config.base_url.clone())
        //.description(crate::config::description())
        // should only change when content has changed
        .last_build_date(Some(
            build_time_local!("%a, %d %b %Y %H:%M:%S %z").to_owned(),
        ))
        .language(config.language.clone())
        .ttl("600".to_owned())
        .generator(generator());
    channel
}

pub fn feed(config: &crate::Config) {
    let mut channel = &mut channel_builder(config);
    let output = Output::new(&config.output);
    for article in Article::from_dir(&config.articles).unwrap() {
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
    channel.build().write_to(output.feed()).unwrap();
}
