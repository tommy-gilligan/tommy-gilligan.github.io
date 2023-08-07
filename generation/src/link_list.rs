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
