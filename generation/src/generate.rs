use build_time::build_time_local;
use rss::{ChannelBuilder, ItemBuilder};
use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    path::Path,
};

pub fn main() {
    let mut pages: Vec<(String, String)> = Vec::new();
    let mut binding = ChannelBuilder::default();

    let mut channel = binding
        .title(crate::config::title())
        .link(crate::config::base_url())
        .description(crate::config::description())
        // should only change when content has changed
        .last_build_date(Some(
            build_time_local!("%a, %d %b %Y %H:%M:%S %z").to_string(),
        ))
        .language(crate::config::language())
        .ttl("600".to_string())
        .generator(crate::config::generator());

    for mut file in crate::page::Page::from_dir("./pages/").unwrap() {
        let footer = crate::history::History {
            commits: file.history(),
        }
        .to_string();
        let body = file.body();
        let output = crate::layout::Layout {
            title: "My Blog",
            body: &body,
            footer: &footer,
            language: &crate::config::language(),
            author: &crate::config::authors()[0],
            page_title: Some(&file.title()),
            description: &file.description(),
            style: &crate::style::style(),
        }
        .to_string();

        let output_dir = Path::new("./_site");
        create_dir_all(output_dir).unwrap();

        pages.push((
            file.output_path(Path::new("."), "html")
                .display()
                .to_string(),
            file.title(),
        ));
        let mut binding = ItemBuilder::default();
        let item = binding
            .title(file.title())
            .description(file.description())
            .pub_date(
                file.published_at()
                    .format("%a, %d %b %Y %H:%M:%S %z")
                    .to_string(),
            );

        channel = channel.item(item.build());
        let mut output_file = File::create(file.output_path(output_dir, "html")).unwrap();
        output_file.write_all(output.as_bytes()).unwrap();
    }

    let output_dir = Path::new("./_site");
    create_dir_all(output_dir).unwrap();

    let output = crate::layout::Layout {
        title: "My Blog",
        body: &crate::link_list::LinkList { links: pages }.to_string(),
        language: &crate::config::language(),
        page_title: None,
        footer: "",
        author: &crate::config::authors()[0],
        description: &crate::config::description(),
        style: &crate::style::style(),
    }
    .to_string();
    let mut output_file = File::create(output_dir.clone().join("index.html")).unwrap();
    output_file.write_all(output.as_bytes()).unwrap();

    let channel = channel.build();
    let output_file = File::create(output_dir.clone().join("pages.xml")).unwrap();
    channel.write_to(output_file).unwrap();
}
