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

#[must_use]
pub fn frontmatter(contents: &str) -> Frontmatter {
    let dom = tl::parse(contents, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let element = dom
        .get_element_by_id("frontmatter")
        .expect("Failed to find element")
        .get(parser)
        .unwrap()
        .inner_text(parser);
    toml::from_str(&element).unwrap()
}
