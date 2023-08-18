use generation::{crawl::Crawler, output::Output};
use url::{Position, Url};

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "_site")]
    output: String,
    #[arg(short, long)]
    base_url: Url,
}

pub fn crawl(config: &Args) {
    let mut sitemap = Output::new(&config.output).sitemap().create();

    for url in Crawler::new() {
        sitemap.push(
            &format!("{}/{}", config.base_url, &url[Position::BeforePath..])
                .parse()
                .unwrap(),
        );
    }
}
