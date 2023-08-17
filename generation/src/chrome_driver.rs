use std::{fs::File, io::prelude::*, path::Path};

use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use serde_json::json;
use std::io::BufReader;
use thirtyfour::{
    extensions::cdp::ChromeDevTools, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver,
};
use url::Url;

pub struct ChromeDriver {
    web_driver: WebDriver,
    dev_tools: ChromeDevTools,
}

impl ChromeDriver {
    pub async fn new() -> Self {
        let mut caps = DesiredCapabilities::chrome();
        // GPU doesn't get used on GH runner
        caps.set_disable_gpu().unwrap();
        let web_driver = WebDriver::new("http://127.0.0.1:9515", caps).await.unwrap();
        Self {
            web_driver: web_driver.clone(),
            dev_tools: ChromeDevTools::new(web_driver.handle),
        }
    }

    pub async fn screenshot(&mut self, path: &Path) {
        let screenshot_as_base64 = self
            .dev_tools
            .execute_cdp_with_params(
                "Page.captureScreenshot",
                json!({"captureBeyondViewport": true}),
            )
            .await
            .unwrap();

        File::create(path)
            .unwrap()
            .write_all(
                &general_purpose::STANDARD
                    .decode(screenshot_as_base64["data"].as_str().unwrap().as_bytes())
                    .unwrap(),
            )
            .unwrap();
    }

    pub async fn goto(&mut self, url: Url) {
        let mut rewritten_url = url.clone();
        rewritten_url.set_host(Some("localhost")).unwrap();
        rewritten_url.set_port(Some(62394)).unwrap();
        rewritten_url.set_scheme("http").unwrap();

        self.web_driver
            .goto(rewritten_url.to_string())
            .await
            .unwrap();
    }

    pub async fn quit(self) {
        self.web_driver.quit().await.unwrap();
        let ignore_re =
            Regex::new(r"alternate_error_pages|org\.freedesktop\.portal\.Settings\.Read").unwrap();
        let path = Path::new("chromedriver.log");
        let f = File::open(path).unwrap();

        if let Some(error_line) = BufReader::new(f)
            .lines()
            .map(std::result::Result::unwrap)
            .filter(|l| l.contains("error"))
            .find(|l| !ignore_re.is_match(l))
        {
            panic!(
                "Matched error in {:?} line: {}",
                path.file_name(),
                error_line
            );
        }
    }
}
