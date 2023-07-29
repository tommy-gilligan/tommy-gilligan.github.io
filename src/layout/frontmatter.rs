use serde::Deserialize;
use markdown::{mdast::{Node, Yaml, Toml}, ParseOptions};

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frontmatter {
    pub title: String,
}

pub fn frontmatter(contents: &str, parse_options: &ParseOptions) -> Frontmatter {
    if let Node::Yaml(Yaml { value, .. }) =
        &markdown::to_mdast(contents, parse_options)
            .unwrap()
            .children()
            .unwrap()[0]
    {
        return serde_yaml::from_str(value).unwrap();
    } else if let Node::Toml(Toml { value, .. }) =
        &markdown::to_mdast(contents, parse_options)
            .unwrap()
            .children()
            .unwrap()[0]
    {
        return toml::from_str(value).unwrap();
    }
    panic!()
}
