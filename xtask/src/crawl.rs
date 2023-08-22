use generation::{crawl::Crawler, output::Output};

use std::net::IpAddr;

use std::net::SocketAddr;
use tokio::net::TcpListener;

use generation::tokiort::TokioIo;
use url::Url;

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "_site")]
    output: String,
    #[arg(short, long)]
    base_url: Url,
}

pub async fn crawl(config: &Args) {
    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    let addr: SocketAddr = (ip, 0).into();
    let listener = TcpListener::bind(addr).await.unwrap();

    let local_addr = listener.local_addr().unwrap();
    let mut sitemap = Output::new(&config.output).sitemap().create();

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

    let mut crawler = Crawler::new(local_addr);
    crawler.push(config.base_url.clone());
    let origin = config.base_url.origin();
    let urls: std::collections::HashSet<Url> = crawler
        .filter(|url| return url.origin() == origin)
        .map(|mut v| {
            v.set_fragment(None);
            return v;
        })
        .collect();
    for url in urls {
        sitemap.push(&url);
    }
}
