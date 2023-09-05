use toolkit::{crawl::Crawler, output::Output};

#[tokio::main]
async fn main() {
    let local_addr = toolkit::serve::run(None).await;

    let mut sitemap = Output::sitemap().create();
    for url in Crawler::new(&local_addr.1).await {
        sitemap.push(&url);
    }
}
