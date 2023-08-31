use toolkit::{crawl::Crawler, output::Output};

#[tokio::main]
async fn main() {
    let local_addr = toolkit::serve::run().await;

    let mut sitemap = Output::sitemap().create();
    for url in Crawler::new(&local_addr).await {
        sitemap.push(&url);
    }
}
