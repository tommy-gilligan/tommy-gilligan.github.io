use markdown::{
    mdast::{Node, Toml},
    ParseOptions,
};
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    #[serde(with = "toml_datetime_compat")]
    pub published: chrono::DateTime<chrono::FixedOffset>,
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
