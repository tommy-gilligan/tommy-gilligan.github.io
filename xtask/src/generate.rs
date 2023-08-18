use std::process::{Child, Command};

use url::Url;
mod articles;
mod feed;
mod index;

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
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

pub fn run() -> Child {
    Command::new("cargo")
        .arg("xtask")
        .arg("generate")
        .arg("--error")
        .spawn()
        .expect("failed to execute process")
}

pub fn generate(config: &Args) {
    feed::feed(config);
    index::render(config);
    articles::render(config);
}
