use std::env::args;

use toolkit::{article, index, rss};

#[tokio::main]
async fn main() {
    let base_url = args().nth(1).unwrap().parse().unwrap();
    rss::feed(&base_url);
    index::render();
    article::render();
    toolkit::prettier::run();
}
