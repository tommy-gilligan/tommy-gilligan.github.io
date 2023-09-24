use std::env::args;

use toolkit::{article, asset};

#[tokio::main]
async fn main() {
    article::render();
    asset::copy();
    // toolkit::prettier::run();
    eprintln!("Finished generating");
}
