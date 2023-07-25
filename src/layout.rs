use serde::Deserialize;

mod markdown_options;
use markdown_options::MARKDOWN_OPTIONS;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frontmatter {
    pub title: String,
}

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
    if let markdown::mdast::Node::Yaml(markdown::mdast::Yaml { value, .. }) =
        &markdown::to_mdast(contents, &MARKDOWN_OPTIONS.parse)
            .unwrap()
            .children()
            .unwrap()[0]
    {
        return serde_yaml::from_str(value).unwrap();
    }
    if let markdown::mdast::Node::Toml(markdown::mdast::Toml { value, .. }) =
        &markdown::to_mdast(contents, &MARKDOWN_OPTIONS.parse)
            .unwrap()
            .children()
            .unwrap()[0]
    {
        return toml::from_str(value).unwrap();
    }
    panic!()
}

pub fn render(contents: &str) -> String {
    Layout {
        frontmatter: frontmatter(contents),
        content: &markdown::to_html_with_options(contents, &MARKDOWN_OPTIONS).unwrap(),
    }
    .to_string()
}
