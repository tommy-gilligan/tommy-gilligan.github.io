use generation::{crawl::Crawler, output::Output};

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "_site")]
    output: String,
    #[arg(short, long)]
    host: String,
}

pub fn crawl(config: &Args) {
    let mut sitemap = Output::new(&config.output).sitemap().create();

    for mut url in Crawler::new() {
        url.set_host(Some(&config.host)).unwrap();
        url.set_port(None).unwrap();
        sitemap.push(&url);
    }
}
