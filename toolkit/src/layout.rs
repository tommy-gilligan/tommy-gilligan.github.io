use askama::Template;

#[derive(Template, Default)]
#[template(path = "layout.html")]
pub struct Layout<'a> {
    pub description: &'a str,
    pub body: &'a str,
    pub lang: &'a str,
    pub sitemap: &'a str,
    pub title: &'a str,
    pub page_title: Option<&'a str>,
}
