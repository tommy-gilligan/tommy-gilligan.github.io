use chrono::{DateTime, Datelike, Utc};
use ordinal::Ordinal;
use url::Url;

const WEB_REPO_URL: &str = "https://github.com/tomgilligan/tomgilligan.github.io/commit/";

fn url_for_sha(sha: &str) -> Url {
    let web_repo_url: Url = WEB_REPO_URL.parse().unwrap();
    web_repo_url.join(sha).unwrap()
}

fn format_timestamp(dt: DateTime<Utc>) -> String {
    format!(
        "{}, {} of {}",
        dt.format("%A"),
        Ordinal(dt.date_naive().day()),
        dt.format("%B, %Y"),
    )
}

markup::define! {
    History(commits: Vec<(DateTime<Utc>, String, String, String)>) {
        ol.commits {
            @for (_index, commit) in commits.iter().rev().enumerate() {
                li.commit {
                    span {
                        a [href = url_for_sha(&commit.3).to_string()] {
                            @commit.1
                        }
                        br;
                        @format_timestamp(commit.0)
                    }
                }
            }
        }
    }
}
