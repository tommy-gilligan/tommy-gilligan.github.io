#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use base64::{engine::general_purpose, Engine as _};
use serde_json::json;
use sitemap::{
    reader::{SiteMapEntity, SiteMapReader},
    structs::{Location, UrlEntry},
};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};
use thirtyfour::{extensions::cdp::ChromeDevTools, prelude::*};
use tokio::process::Command;

const PATH: &str = "target/debug/serve";

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
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://127.0.0.1:9515", caps).await.unwrap();
    let dev_tools = ChromeDevTools::new(driver.handle.clone());

    let file = File::open(output_dir.clone().join("sitemap.xml")).expect("Unable to open file.");
    let parser = SiteMapReader::new(file);
    for entity in parser {
        if let SiteMapEntity::Url(UrlEntry {
            loc: Location::Url(mut url),
            ..
        }) = entity
        {
            url.set_host(Some("localhost")).unwrap();
            url.set_port(Some(62394)).unwrap();
            url.set_scheme("http").unwrap();
            assert!(server_child.id().is_some());
            driver.goto(url.to_string()).await.unwrap();
            let screenshot_as_base64 = dev_tools
                .execute_cdp_with_params(
                    "Page.captureScreenshot",
                    json!({"captureBeyondViewport": true}),
                )
                .await
                .unwrap();
            let path = url.path().strip_prefix('/').unwrap();
            let mut joined_path = screenshots_dir.join(path);
            joined_path = joined_path.with_extension("png");
            let mut screenshot_file = File::create(joined_path).unwrap();
            screenshot_file
                .write_all(
                    &general_purpose::STANDARD
                        .decode(screenshot_as_base64["data"].as_str().unwrap().as_bytes())
                        .unwrap(),
                )
                .unwrap();
        }
    }

    server_child.start_kill().unwrap();
    server_child.wait().await.unwrap();

    driver.quit().await.unwrap();
}
