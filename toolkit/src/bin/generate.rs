use std::env::args;

use toolkit::{article, asset, index, rss};

#[tokio::main]
async fn main() {
    // rss::feed(&base_url);
    index::render();
    article::render();
    asset::copy();
    toolkit::prettier::run();
    eprintln!("Finished generating");
}
