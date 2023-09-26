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
    ArticleList<'a>(_articles: Vec<crate::article::Article>, formatted_code: &'a str) {
        @markup::raw(formatted_code)
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
