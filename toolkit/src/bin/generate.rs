use clap::Parser;

use toolkit::{feed, index, my_article};
use url::Url;

fn main() {
    let config = toolkit::config::Config::parse();
    feed::feed(&config);
    index::render(&config);
    my_article::render(&config);
}
