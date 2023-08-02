markup::define! {
    LinkList(pages: Vec<(String, String)>) {
        @for (href, text) in pages.iter() {
            a [href = href] {
                @text
            }
        }
    }
}
