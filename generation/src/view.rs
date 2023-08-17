use crate::{ditto::Ditto, github::Remote, srcset::srcset};
use chrono::{Datelike, TimeZone, Utc};
use git2::Commit;
use ordinal::Ordinal;
use url::Url;

fn format_commit(remote: &Remote, commit: &Commit) -> [String; 4] {
    let datetime = Utc.timestamp_opt(commit.time().seconds(), 0).unwrap();
    [
        CommitLink { remote, commit }.to_string(),
        datetime.format("%H:%M").to_string(),
        Ordinal(datetime.date_naive().day()).to_string(),
        datetime.format(" of %B, %Y").to_string(),
    ]
}

markup::define! {
    CommitLink<'a>(remote: &'a Remote, commit: &'a Commit<'a>) {
        a [href = remote.page_for(commit).to_string()] {
            @commit.message()
        }
    }
}

markup::define! {
    Link<'a>(href: &'a str, text: &'a str, favicon: Option<&'a str>) {
        a [href = href] {
            @if favicon.is_some() {
                img [src = favicon.unwrap(), alt = format!("Favicon for {}", text)];
            }
            @text
        }
    }
}

markup::define! {
    Footer(author: String, revisions: String) {
        @markup::raw(author)
        @markup::raw(revisions)
    }
}

markup::define! {
    LinkList(links: Vec<(String, String)>) {
        ol {
            @for (href, text) in links.iter() {
                li {
                    a [href = href] {
                        @text
                    }
                }
            }
        }
    }
}

markup::define! {
    CodeContainer<'a>(
        formatted_code: &'a str,
        language: &'a str,
    ) {
        pre {
            code.{format!("language-{}", language)} {
                @markup::raw(formatted_code)
            }
        }
    }
}

markup::define! {
    History<'a>(remote: &'a Remote, commits: Vec<Commit<'a>>) {
        table.commits {
            tbody {
                @for [a, b, c, d] in Ditto::new(commits.iter().map(|c| format_commit(remote, c))) {
                    tr {
                        td { @a } td { @b } td { @c } td { @d }
                    }
                }
            }
            caption {
                "Revisions"
            }
        }
    }
}

markup::define! {
    Author<F>(name: String, image_url_for: F, social_links: Vec<(String, Url)>) where F: Fn(u16) -> Url {
        div.author {
            img[
                loading = "lazy",
                fetchpriority = "low",
                decoding = "aync",
                src = image_url_for(120).to_string(),
                srcset = srcset(image_url_for, 120),
                alt = &name
            ];
            span {
                div {
                    @name
                    br;
                    @for (service_name, url) in social_links {
                        a[href = url.to_string()] {
                            @service_name
                        }
                        br;
                    }
                }
            }
        }
    }
}
