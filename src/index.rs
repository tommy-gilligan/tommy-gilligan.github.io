markup::define! {
    Index(pages: Vec<(String, String)>) {
        @markup::doctype()
        html[lang = "en-AU"] {
            head {
                meta[charset = "utf-8"];
                meta["http-equiv" = "Content-Security-Policy", content = "default-src 'none'; script-src 'none'; script-src-elem 'self'; script-src-attr 'none'; style-src 'none'; style-src-elem 'unsafe-inline'; style-src-attr 'none'; img-src 'self' data:; font-src 'none'; connect-src 'none'; media-src 'none'; object-src 'none'; prefetch-src *; child-src 'none'; frame-src 'none'; worker-src 'none'; frame-ancestors 'none'; form-action 'none'; upgrade-insecure-requests; block-all-mixed-content; sandbox allow-same-origin allow-scripts allow-top-navigation; base-uri 'self'; manifest-src 'self'"];
                title { "My Blog" }
                style { @include_str!("layout.css") }
                link[rel = "alternate", r#type = "application/rss+xml", href = "pages.xml", title = "My Blog"];
            }
            body {
                header {
                    h1 { "My Blog" }
                    h2 { "Pages" }
                }
                main {
                    @for (href, text) in pages.iter() {
                        a [href = href] {
                            @text
                        }
                    }
                }
            }
        }
    }
}

pub fn render(pages: Vec<(String, String)>) -> String {
    Index { pages }.to_string()
}
