#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use rss::{ChannelBuilder, ItemBuilder};
use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    path::Path,
};

mod index;
mod layout;
mod source_file;

use build_time::build_time_local;
use git_version::git_version;

fn language() -> String {
    "en-AU".to_string()
}

fn title() -> String {
    "My Blog".to_string()
}

// fn strip_email(mut author: String) -> String {
//     if let Some(start_index) = author.find('<') {
//         if let Some(end_index) = author.rfind('>') {
//             if end_index > start_index {
//                 author.replace_range(start_index..=end_index, "")
//             }
//         }
//     }
//     assert!(author.find('@').is_none(), );
//
//     author.trim().to_string()
// }
//
// fn authors() -> Vec<String> {
//     if !env!("CARGO_PKG_AUTHORS").is_empty() {
//         env!("CARGO_PKG_AUTHORS").split(':').map(|author| strip_email(author.to_string())).collect()
//     } else {
//         panic!()
//     }
// }

fn generator() -> String {
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

fn homepage() -> String {
    assert!(!env!("CARGO_PKG_HOMEPAGE").is_empty(),);
    env!("CARGO_PKG_HOMEPAGE").to_string()
}

use url::Url;
fn base_url() -> Url {
    homepage().parse().unwrap()
}

fn description() -> String {
    assert!(!env!("CARGO_PKG_DESCRIPTION").is_empty(),);
    env!("CARGO_PKG_DESCRIPTION").to_string()
}

fn main() {
    let mut pages: Vec<(String, String)> = Vec::new();
    let mut binding = ChannelBuilder::default();
    let mut channel = binding
        .title(title())
        .link(base_url())
        .description(description())
        // should only change when content has changed
        .last_build_date(Some(
            build_time_local!("%a, %d %b %Y %H:%M:%S %z").to_string(),
        ))
        .language(language())
        .ttl("600".to_string())
        .generator(generator());
    for file in source_file::SourceFile::from_dir(Path::new(".")).unwrap() {
        let body = file.body();
        let frontmatter = file.frontmatter();
        let output = layout::render(&body, frontmatter);

        let output_dir = Path::new("./_site");
        create_dir_all(output_dir).unwrap();

        pages.push((
            file.output_path(Path::new("."), "html")
                .display()
                .to_string(),
            file.frontmatter().title,
        ));
        let mut binding = ItemBuilder::default();
        let item = binding
            .title(file.frontmatter().title)
            .description(file.frontmatter().description)
            .author(file.frontmatter().author)
            .pub_date(
                file.frontmatter()
                    .published_at
                    .format("%a, %d %b %Y %H:%M:%S %z")
                    .to_string(),
            );

        channel = channel.item(item.build());
        let mut output_file = File::create(file.output_path(output_dir, "html")).unwrap();
        output_file.write_all(output.as_bytes()).unwrap();
    }

    let output_dir = Path::new("./_site");
    create_dir_all(output_dir).unwrap();
    let output = index::render(pages);

    let mut output_file = File::create(output_dir.clone().join("index.html")).unwrap();

    output_file.write_all(output.as_bytes()).unwrap();
    let channel = channel.build();

    let output_file = File::create(output_dir.clone().join("pages.xml")).unwrap();
    channel.write_to(output_file).unwrap(); // // write to the channel to a writer
}
