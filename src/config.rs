use git_version::git_version;
use url::Url;

pub fn language() -> String {
    "en-AU".to_string()
}

pub fn title() -> String {
    "My Blog".to_string()
}

pub fn strip_email(mut author: String) -> String {
    if let Some(start_index) = author.find('<') {
        if let Some(end_index) = author.rfind('>') {
            if end_index > start_index {
                author.replace_range(start_index..=end_index, "");
            }
        }
    }
    assert!(author.find('@').is_none(),);

    author.trim().to_string()
}

pub fn authors() -> Vec<String> {
    if env!("CARGO_PKG_AUTHORS").is_empty() {
        panic!()
    } else {
        env!("CARGO_PKG_AUTHORS")
            .split(':')
            .map(|author| strip_email(author.to_string()))
            .collect()
    }
}

pub fn generator() -> String {
    if env!("CARGO_PKG_REPOSITORY").is_empty() {
        format!(
            "{} version: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    } else {
        format!(
            "{} version: {}",
            env!("CARGO_PKG_REPOSITORY"),
            git_version!()
        )
    }
}

pub fn homepage() -> String {
    assert!(!env!("CARGO_PKG_HOMEPAGE").is_empty(),);
    env!("CARGO_PKG_HOMEPAGE").to_string()
}

pub fn base_url() -> Url {
    homepage().parse().unwrap()
}

pub fn description() -> String {
    assert!(!env!("CARGO_PKG_DESCRIPTION").is_empty(),);
    env!("CARGO_PKG_DESCRIPTION").to_string()
}
