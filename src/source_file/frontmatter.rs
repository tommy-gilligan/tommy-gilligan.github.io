use serde::Deserialize;
use markdown::{mdast::{Node, Toml}, ParseOptions};

const DEFAULT_AUTHOR: &str = "Tommy Gilligan";

fn default_author() -> String {
    DEFAULT_AUTHOR.to_string()
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    #[serde(default = "default_author")]
    pub author: String,
    pub published_at: toml::value::Datetime
}

pub fn frontmatter(contents: &str, parse_options: &ParseOptions) -> Frontmatter {
    if let Node::Toml(Toml { value, .. }) =
        &markdown::to_mdast(contents, parse_options)
            .unwrap()
            .children()
            .unwrap()[0]
    {
        return toml::from_str(value).unwrap();
    }
    // refuse to deal with Yaml because this is a Rust project
    panic!()
}
