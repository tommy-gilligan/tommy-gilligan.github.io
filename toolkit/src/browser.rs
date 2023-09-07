use std::{fs::File, io::prelude::*, path::Path};

use base64::{engine::general_purpose, Engine as _};
use regex::Regex;

use serde_json::json;

use std::io::BufReader;
use std::net::SocketAddr;
use thirtyfour::WebElement;
use thirtyfour::{
    extensions::cdp::ChromeDevTools, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver,
};

use thirtyfour::prelude::ElementQueryable;
use thirtyfour::By;
use url::Url;

pub struct Browser {
    web_driver: WebDriver,
    dev_tools: ChromeDevTools,
    server_addr: SocketAddr,
    url: Option<Url>,
}

impl Browser {
    pub async fn new(server_addr: &SocketAddr) -> Self {
        let mut caps = DesiredCapabilities::chrome();
        // GPU doesn't get used on GH runner
        caps.set_disable_gpu().unwrap();
        let web_driver = WebDriver::new("http://127.0.0.1:9515", caps).await.unwrap();
        Self {
            server_addr: *server_addr,
            web_driver: web_driver.clone(),
            dev_tools: ChromeDevTools::new(web_driver.handle),
            url: None,
        }
    }

    pub async fn main(&mut self) -> WebElement {
        self.web_driver
            .query(By::Css("main"))
            .first()
            .await
            .unwrap()
    }

    pub async fn children(&mut self) -> Vec<WebElement> {
        self.web_driver
            .query(By::Css("main > * > *"))
            .all_from_selector()
            .await
            .unwrap()
    }

    pub async fn screenshot(&mut self) {
        let url = self.url.clone();

        let screenshot_as_base64 = self
            .dev_tools
            .execute_cdp_with_params(
                "Page.captureScreenshot",
                json!({"captureBeyondViewport": true}),
            )
            .await
            .unwrap();

        File::create(
            Path::new(crate::SCREENSHOTS)
                .join(url.unwrap().path_segments().unwrap().last().unwrap())
                .with_extension("png"),
        )
        .unwrap()
        .write_all(
            &general_purpose::STANDARD
                .decode(screenshot_as_base64["data"].as_str().unwrap().as_bytes())
                .unwrap(),
        )
        .unwrap();
    }

    pub async fn goto(&mut self, url: &Url) {
        self.url = Some(url.clone());
        let mut rewritten_url = url.clone();
        rewritten_url
            .set_host(Some(&self.server_addr.ip().to_string()))
            .unwrap();
        rewritten_url
            .set_port(Some(self.server_addr.port()))
            .unwrap();
        rewritten_url.set_scheme("http").unwrap();

        self.web_driver
            .goto(rewritten_url.to_string())
            .await
            .unwrap();
    }

    pub async fn links(&mut self) -> Vec<Url> {
        futures::future::join_all(
            self.web_driver
                .find_all(thirtyfour::By::Css(":any-link"))
                .await
                .unwrap()
                .iter()
                .map(|link| async { link.prop("href").await.unwrap().unwrap().parse().unwrap() }),
        )
        .await
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
