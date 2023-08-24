use clap::Parser;

use url::Url;

mod articles;
mod feed;
mod index;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub base_url: Url,
    #[arg(short, long, default_value = "_site")]
    pub output: String,
    #[arg(short, long, default_value = "cache")]
    pub cache: String,
    #[arg(short, long, default_value = "articles")]
    pub articles: String,
    #[arg(short, long, default_value = "Tommy's Blog")]
    pub title: String,
    #[arg(short, long, default_value = "en-AU")]
    pub language: String,
}

fn main() {
    let config = Config::parse();
    feed::feed(&config);
    index::render(&config);
    articles::render(&config);
}
