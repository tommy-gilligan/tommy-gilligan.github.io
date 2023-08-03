#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use build_time::build_time_local;
use rss::{ChannelBuilder, ItemBuilder};
use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    path::Path,
};

mod config;
mod layout;
mod link_list;
mod source_file;
mod syntax_highlighting;

fn main() {
    let mut pages: Vec<(String, String)> = Vec::new();
    let mut binding = ChannelBuilder::default();
    let mut channel = binding
        .title(config::title())
        .link(config::base_url())
        .description(config::description())
        // should only change when content has changed
        .last_build_date(Some(
            build_time_local!("%a, %d %b %Y %H:%M:%S %z").to_string(),
        ))
        .language(config::language())
        .ttl("600".to_string())
        .generator(config::generator());

    for mut file in source_file::SourceFile::from_dir(Path::new(".")).unwrap() {
        let body = file.body();
        let frontmatter = file.frontmatter();
        let output = layout::Layout {
            title: "My Blog",
            body: &body,
            language: &config::language(),
            page_title: Some(&frontmatter.title),
            author: &frontmatter.author,
            description: &frontmatter.description,
        }
        .to_string();

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

    let output = layout::Layout {
        title: "My Blog",
        body: &link_list::LinkList { links: pages }.to_string(),
        language: &config::language(),
        page_title: None,
        author: &config::authors()[0],
        description: &config::description(),
    }
    .to_string();
    let mut output_file = File::create(output_dir.clone().join("index.html")).unwrap();
    output_file.write_all(output.as_bytes()).unwrap();

    let channel = channel.build();
    let output_file = File::create(output_dir.clone().join("pages.xml")).unwrap();
    channel.write_to(output_file).unwrap();
}
