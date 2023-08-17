use crate::serve;
use generation::{chrome_driver::ChromeDriver, output::Output};
use std::{fs::create_dir_all, path::Path};

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args;

pub async fn screenshot(_config: &Args) {
    let screenshots_dir = Path::new("./screenshots");
    create_dir_all(screenshots_dir).unwrap();

    let server_child = serve::serve(&serve::Args);
    let abort_handle = server_child.abort_handle();
    server_child.await.unwrap();
    let mut driver = ChromeDriver::new().await;

    for url in Output::new("./_site").sitemap().open() {
        assert!(!abort_handle.is_finished());
        driver.goto(url.clone()).await;

        let path = url.path().strip_prefix('/').unwrap();
        let mut joined_path = screenshots_dir.join(path);
        joined_path = joined_path.with_extension("png");

        driver.screenshot(&joined_path).await;
    }

    abort_handle.abort();

    driver.quit().await;
}
