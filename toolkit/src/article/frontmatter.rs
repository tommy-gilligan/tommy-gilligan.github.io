use markdown::{
    mdast::{Node, Toml},
    ParseOptions,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    #[serde_as(as = "Option<toml_datetime_compat::TomlDateTime>")]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub published: Option<bool>,
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
