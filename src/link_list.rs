markup::define! {
    LinkList(links: Vec<(String, String)>) {
        @for (href, text) in links.iter() {
            a [href = href] {
                @text
            }
        }
    }
}
