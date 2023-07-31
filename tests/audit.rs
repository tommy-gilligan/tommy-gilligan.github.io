#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use thirtyfour::extensions::cdp::ChromeDevTools;
use thirtyfour::prelude::*;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(hyper_staticfile::Static::new("_site")
        .serve(req)
        .await
        .unwrap())
}

#[tokio::test]
async fn test_audit() -> WebDriverResult<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = Server::bind(&addr);
    let serve = server.serve(make_service);
    let local_addr = serve.local_addr();

    tokio::spawn(async move {
        if let Err(e) = serve.await {
            eprintln!("server error: {e}");
        }
    });

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://127.0.0.1:9515", caps).await?;

    let dev_tools = ChromeDevTools::new(driver.handle.clone());
    dev_tools.execute_cdp("Audits.enable").await?;

    driver.goto(format!("http://{local_addr}")).await?;
    driver.quit().await?;

    Ok(())
}
