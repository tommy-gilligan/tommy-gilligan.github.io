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

pub fn run(base_url: &Url) -> Child {
    Command::new(
        std::path::Path::new(env!("OUT_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(env!("CARGO_BIN_NAME")),
    )
    .arg("generate")
    .arg("--base-url")
    .arg(&base_url.to_string())
    .spawn()
    .expect("failed to execute process")
}

pub fn generate(config: &Args) {
    feed::feed(config);
    index::render(config);
    articles::render(config);
}
