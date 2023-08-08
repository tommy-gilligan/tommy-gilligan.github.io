
use url::Url;

pub fn homepage() -> Url {
    assert!(!env!("CARGO_PKG_HOMEPAGE").is_empty(),);
    let mut url: Url = env!("CARGO_PKG_HOMEPAGE").to_string().parse().unwrap();
    url.set_scheme("https").unwrap();
    assert_eq!(url.scheme(), "https");
    url
}

pub fn base_url() -> Url {
    homepage()[..url::Position::BeforePath].parse().unwrap()
}

pub fn description() -> String {
    assert!(!env!("CARGO_PKG_DESCRIPTION").is_empty(),);
    env!("CARGO_PKG_DESCRIPTION").to_string()
}
