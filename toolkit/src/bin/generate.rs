use std::env::args;

use toolkit::{article, asset, index, rss};

#[tokio::main]
async fn main() {
    eprintln!("Starting to generate");
    let base_url = args().nth(1).unwrap().parse().unwrap();
    rss::feed(&base_url);
    index::render();
    article::render();
    asset::copy();
    toolkit::prettier::run();
    eprintln!("Successfully finished generating");
}
