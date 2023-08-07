#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use thirtyfour::{extensions::cdp::ChromeDevTools, prelude::*};
use tokio::process::Command;
const PATH: &str = "target/debug/serve";

#[tokio::test]
async fn test_audit() -> WebDriverResult<()> {
    let server_child = Command::new(PATH)
        .spawn()
        .expect("failed to execute process");
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://127.0.0.1:9515", caps).await?;

    let dev_tools = ChromeDevTools::new(driver.handle.clone());
    dev_tools.execute_cdp("Audits.enable").await?;

    assert!(server_child.id().is_some());
    driver.goto("http://127.0.0.1:62394".to_string()).await?;
    server_child.start_kill().unwrap();
    server_child.wait().await.unwrap();
    driver.quit().await?;

    Ok(())
}
