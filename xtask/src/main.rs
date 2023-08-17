#![feature(async_closure)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

mod crawl;
mod generate;
mod screenshot;
mod serve;
mod visual_diff;
mod watch;

use clap::Parser;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(bin_name = "xtask")]
enum Cli {
    Crawl(crawl::Args),
    Generate(generate::Args),
    Screenshot(screenshot::Args),
    Serve(serve::Args),
    VisualDiff(visual_diff::Args),
    Watch(watch::Args),
}

#[tokio::main]
async fn main() {
    match Cli::parse() {
        Cli::Generate(args) => generate::generate(&args),
        Cli::Crawl(args) => crawl::crawl(&args),
        Cli::Screenshot(args) => screenshot::screenshot(&args).await,
        Cli::Serve(args) => serve::serve(&args).await.unwrap(),
        Cli::VisualDiff(args) => visual_diff::visual_diff(&args),
        Cli::Watch(args) => watch::watch(&args).await,
    }
}
