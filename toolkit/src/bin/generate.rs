use clap::Parser;

use toolkit::{article, feed, index};

#[tokio::main]
async fn main() {
    let config = toolkit::config::Config::parse();
    feed::feed(&config);
    index::render(&config);
    article::render(&config);
}
