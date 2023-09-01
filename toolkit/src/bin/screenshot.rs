use std::{fs::create_dir_all, path::Path};
use toolkit::{browser::Browser, output::Output};

#[tokio::main]
async fn main() {
    create_dir_all(Path::new(toolkit::SCREENSHOTS)).unwrap();

    let address = toolkit::serve::run().await;
    let mut browser = Browser::new(&address.1).await;

    for url in Output::sitemap().open() {
        browser.goto(&url).await;
        browser.screenshot().await;
    }
    browser.quit().await;
}
