#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use sitemap::{
    reader::{SiteMapEntity, SiteMapReader},
    structs::{Location, UrlEntry},
};
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

use tokio::process::Command;

const PATH: &str = "target/debug/serve";

mod chrome_driver;
#[allow(dead_code)]
mod config;

#[tokio::main]
async fn main() {
    let output_dir = Path::new("./_site");
    create_dir_all(output_dir).unwrap();

    let screenshots_dir = Path::new("./screenshots");
    create_dir_all(screenshots_dir).unwrap();

    let mut server_child = Command::new(PATH)
        .spawn()
        .expect("failed to execute process");

    let file = File::open(output_dir.clone().join("sitemap.xml")).expect("Unable to open file.");
    let parser = SiteMapReader::new(file);

    let mut driver = chrome_driver::ChromeDriver::new().await;
    for entity in parser {
        if let SiteMapEntity::Url(UrlEntry {
            loc: Location::Url(url),
            ..
        }) = entity
        {
            assert!(server_child.id().is_some());
            driver.goto(url.clone()).await;

            let path = url.path().strip_prefix('/').unwrap();
            let mut joined_path = screenshots_dir.join(path);
            joined_path = joined_path.with_extension("png");

            driver.screenshot(&joined_path).await;
        }
    }

    server_child.start_kill().unwrap();
    server_child.wait().await.unwrap();

    driver.quit().await;
}
