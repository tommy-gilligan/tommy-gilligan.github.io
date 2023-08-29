use clap::Parser;
use std::path::Path;

use toolkit::{article, feed, index};

#[tokio::main]
async fn main() {
    let config = toolkit::config::Config::parse();
    feed::feed(&config);
    index::render(&config);
    article::render(&config);
    toolkit::prettier::run(Path::new("."));
}
