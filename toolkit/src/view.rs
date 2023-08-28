use crate::srcset::srcset;
use chrono::{Datelike, TimeZone, Utc};
use git2::Commit;
use ordinal::Ordinal;
use url::Url;

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
    ArticleItem<'a>(article: &'a crate::article::Article) {
        // TODO: set ping attribute
        a[href = format!("{}.html", article.file_stem().to_str().unwrap())] {
            @article.title()
        }
        br;
        time[datetime = article.published_at().format("%+").to_string()] {
            @crate::locale::format(article.published_at())
        }
        @if let Some(updated_at) = article.updated_at() {
            br;
            em {
                "Updated: "
                time[datetime = updated_at.format("%+").to_string()] {
                    @crate::locale::format(updated_at)
                }
            }
        }
    }
}

markup::define! {
    ArticleList(articles: Vec<crate::article::Article>) {
        h2 {
            "Articles"
        }
        @for article in articles.iter() {
            div {
                @ArticleItem { article }
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

// validator doesn't like these attributes on img
// loading = "lazy",
// fetchpriority = "low",
// decoding = "aync",

markup::define! {
    Author<F>(name: String, image_url_for: F, social_links: Vec<(String, Url)>) where F: Fn(u16) -> Url {
        div.author {
            img[
                src = image_url_for(120).to_string(),
                srcset = srcset(image_url_for, 120),
                alt = &name
            ];
            span {
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
