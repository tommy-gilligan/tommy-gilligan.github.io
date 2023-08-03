use markdown::{
    mdast::{Node, Toml},
    ParseOptions,
};
use serde::Deserialize;

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
    #[serde(with = "toml_datetime_compat")]
    pub published_at: chrono::DateTime<chrono::FixedOffset>,
}

pub fn frontmatter(contents: &str, parse_options: &ParseOptions) -> Frontmatter {
    if let Ok(mdast) = markdown::to_mdast(contents, parse_options) {
        if let [Node::Toml(Toml { value, .. }), ..] = &mdast.children().unwrap()[..] {
            if let Ok(frontmatter) = toml::from_str(value) {
                return frontmatter;
            }
        }
    }
    unimplemented!("No YAML support planned")
}
