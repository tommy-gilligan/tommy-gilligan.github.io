#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use std::{
    fs::{self, File},
    io::{prelude::*, BufReader},
    process::Command,
};

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Frontmatter {
    pub title: String
}

mod markdown_options;
use crate::markdown_options::MARKDOWN_OPTIONS;

markup::define! {
    Layout<'a>(content: &'a str, frontmatter: Frontmatter) {
        @markup::doctype()
        html {
            head {
                title { @frontmatter.title }
                style { @include_str!("layout.css") }
            }
            body {
                header { h1 { @frontmatter.title } }
                #main { @markup::raw(content) }
                footer { "(c) " 2023 }
            }
        }
    }
}

fn frontmatter(contents: &str) -> Frontmatter {
    match &markdown::to_mdast(contents, &MARKDOWN_OPTIONS.parse).unwrap().children().unwrap()[0] {
        markdown::mdast::Node::Yaml(markdown::mdast::Yaml { value, .. }) => {
            let fm: Frontmatter = serde_yaml::from_str(&value).unwrap();
            return fm
        },
        _ => ()
    }
    panic!()
}

fn main() -> Result<(), String> {
    for entry in fs::read_dir(".")
        .unwrap()
        .map(std::result::Result::unwrap)
        .filter(|e| {
            e.file_type().unwrap().is_file()
                && e.path().extension().is_some()
                && e.path().extension().unwrap().to_str().unwrap() == "md"
        })
    {
        let mut path = entry.path();
        let file = File::open(path.clone()).unwrap();
        path.set_extension("html");
        let mut output_file = File::create(path.clone()).unwrap();

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        output_file.write_all(
            format!(
                "{}",
                Layout {
                    frontmatter: frontmatter(&contents),
                    content: &markdown::to_html_with_options(
                        &contents,
                        &MARKDOWN_OPTIONS
                    )?
                }
            ).as_bytes()
        ).unwrap();
        output_file.sync_all().unwrap();
        Command::new("npx")
            .args(["prettier", &path.into_os_string().into_string().unwrap(), "--write"])
            .output()
            .expect("failed to execute process");
    }

    Ok(())
}
