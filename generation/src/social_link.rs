use url::Url;

markup::define! {
    SocialLinkView(service_name: String, url: Url) {
        a[url = url] {
            @service_name
        }
    }
}
