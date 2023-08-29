use clap::Parser;
use tokio::net::TcpListener;
use toolkit::tokiort::TokioIo;
use toolkit::{crawl::Crawler, output::Output};
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value = "_site")]
    output: String,
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

    let output = config.output.clone();
    let local_addr = listener.local_addr().unwrap();
    tokio::task::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);

            let service = toolkit::serve::Service::new(output.clone().into());
            tokio::task::spawn(async move {
                hyper::server::conn::http1::Builder::new()
                    .serve_connection(io, service)
                    .await
                    .unwrap();
            });
        }
    });

    let mut crawler = Crawler::new(toolkit::chrome_driver::ChromeDriver::new(&local_addr).await);
    crawler.push(config.base_url.clone());
    let mut sitemap = Output::new(&config.output).sitemap().create();
    for url in crawler {
        sitemap.push(&url);
    }
}
