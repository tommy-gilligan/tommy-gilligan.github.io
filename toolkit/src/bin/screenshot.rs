use std::{fs::create_dir_all, path::Path};
use toolkit::{browser::Browser, output::Output, serve};

#[tokio::main]
async fn main() {
    create_dir_all(Path::new(toolkit::SCREENSHOTS)).unwrap();

    let mut browser = Browser::new(&serve::run().await).await;

    for url in Output::sitemap().open() {
        browser.goto(&url).await;
        browser.screenshot().await;
    }
    browser.quit().await;
}
