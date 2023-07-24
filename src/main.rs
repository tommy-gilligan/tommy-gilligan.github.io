#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use std::{
    fs::{self, File},
    io::{prelude::*, BufReader},
};

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Frontmatter {
    pub title: String
}

const MARKDOWN_OPTIONS: markdown::Options = markdown::Options {
    compile: markdown::CompileOptions {
        allow_dangerous_html: true,
        allow_dangerous_protocol: false,
        default_line_ending: markdown::LineEnding::LineFeed,
        gfm_footnote_back_label: None,
        gfm_footnote_clobber_prefix: None,
        gfm_footnote_label: None,
        gfm_footnote_label_attributes: None,
        gfm_footnote_label_tag_name: None,
        gfm_tagfilter: false,
    },
    parse: markdown::ParseOptions {
        constructs: markdown::Constructs {
            attention: true,
            gfm_autolink_literal: true,
            autolink: false,
            block_quote: true,
            character_escape: true,
            character_reference: false,
            code_indented: true,
            code_fenced: true,
            code_text: true,
            definition: true,
            frontmatter: true,
            gfm_footnote_definition: true,
            gfm_label_start_footnote: true,
            gfm_strikethrough: true,
            gfm_table: true,
            gfm_task_list_item: false,
            hard_break_escape: false,
            hard_break_trailing: false,
            html_text: false,
            html_flow: false,
            heading_setext: false,
            heading_atx: true,
            label_start_image: true,
            label_start_link: true,
            label_end: true,
            list_item: true,
            math_flow: true,
            math_text: true,
            mdx_esm: false,
            mdx_expression_flow: false,
            mdx_expression_text: false,
            mdx_jsx_flow: false,
            mdx_jsx_text: false,
            thematic_break: true,
        },
        gfm_strikethrough_single_tilde: true,
        math_text_single_dollar: true,
        mdx_expression_parse: None,
        mdx_esm_parse: None,
    },
};

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
        let mut output_file = File::create(path).unwrap();

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
    }

    Ok(())
}
