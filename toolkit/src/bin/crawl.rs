use tokio::net::TcpListener;
use toolkit::{crawl::Crawler, output::Output, tokiort::TokioIo};

const OUTPUT: &str = "_site";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::LOCALHOST,
        0,
    ))
    .await
    .unwrap();

    let local_addr = listener.local_addr().unwrap();
    tokio::task::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);

            let service = toolkit::serve::Service::new(OUTPUT.clone().into());
            tokio::task::spawn(async move {
                hyper::server::conn::http1::Builder::new()
                    .serve_connection(io, service)
                    .await
                    .unwrap();
            });
        }
    });

    let mut sitemap = Output::new(OUTPUT).sitemap().create();
    for url in Crawler::new(&local_addr).await {
        sitemap.push(&url);
    }
}
