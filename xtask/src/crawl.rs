use generation::tokiort::TokioIo;
use generation::{crawl::Crawler, output::Output};
use tokio::net::TcpListener;
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
    let mut sitemap = Output::new(&config.output).sitemap().create();
    for url in crawler {
        sitemap.push(&url);
    }
}
