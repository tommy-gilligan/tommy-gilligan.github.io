use crate::ditto::Ditto;
use chrono::{Datelike, TimeZone, Utc};
use git2::Commit;
use ordinal::Ordinal;
use url::Url;

const WEB_REPO_URL: &str = "https://github.com/tomgilligan/tomgilligan.github.io/commit/";

fn url_for_sha(sha: &str) -> Url {
    let web_repo_url: Url = WEB_REPO_URL.parse().unwrap();
    web_repo_url.join(sha).unwrap()
}

markup::define! {
    Link<'a>(commit: &'a Commit<'a>) {
        a [href = url_for_sha(&commit.id().to_string()).to_string()] {
            @commit.message()
        }
    }
}

fn format_commit(commit: &Commit) -> [String; 4] {
    let datetime = Utc.timestamp_opt(commit.time().seconds(), 0).unwrap();
    [
        Link { commit: commit }.to_string(),
        datetime.format("%H:%M").to_string(),
        Ordinal(datetime.date_naive().day()).to_string(),
        datetime.format(" of %B, %Y").to_string(),
    ]
}

markup::define! {
    History<'a>(commits: Vec<Commit<'a>>) {
        table.commits {
            tbody {
                @for [a, b, c, d] in Ditto::new(commits.iter().map(format_commit)) {
                    tr {
                        td {
                            @a
                        }
                        td {
                            @b
                        }
                        td {
                            @c
                        }
                        td {
                            @d
                        }
                    }
                }
            }
            caption {
                "Revisions"
            }
        }
    }
}
