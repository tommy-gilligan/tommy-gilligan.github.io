use generation::tokiort::TokioIo;

use clap::Parser;
use std::{fs::create_dir_all, path::Path};
use tokio::net::TcpListener;
use url::{Position, Url};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value = "_site")]
    output: String,
    #[arg(short, long, default_value = "screenshots")]
    screenshots: String,
    #[arg(short, long)]
    base_url: Url,
}

#[tokio::main]
async fn main() {
    let config = Config::parse();
    let listener = TcpListener::bind(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::LOCALHOST,
        0,
    ))
    .await
    .unwrap();
    let local_addr = listener.local_addr().unwrap();
    println!("{:?}", local_addr);

    let output = config.output.clone();
    tokio::task::spawn(async move {
        let output = output.clone();
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);

            let service = generation::serve::Service::new(output.clone().into());
            tokio::task::spawn(async move {
                hyper::server::conn::http1::Builder::new()
                    .serve_connection(io, service)
                    .await
                    .unwrap();
            });
        }
    });

    let screenshots_dir = Path::new(&config.screenshots);
    create_dir_all(screenshots_dir).unwrap();

    let mut driver = generation::chrome_driver::ChromeDriver::new().await;

    for url in generation::output::Output::new(&config.output)
        .sitemap()
        .open()
    {
        let url: Url = format!(
            "http://{}:{}{}",
            local_addr.ip(),
            local_addr.port(),
            &url[Position::BeforePath..]
        )
        .parse()
        .unwrap();
        driver.goto(url.clone()).await;

        let path = url.path_segments().unwrap().last().unwrap();
        let mut joined_path = screenshots_dir.join(path);
        joined_path = joined_path.with_extension("png");

        driver.screenshot(&joined_path).await;
    }

    driver.quit().await;
}
