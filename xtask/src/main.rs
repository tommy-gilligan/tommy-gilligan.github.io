#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::io::Write;

use clap::Parser;
use generation::{
    cache::Cache,
    chrome_driver,
    crawl::Crawler,
    layout::{Factory, Layout},
    output::Output,
    page::Page,
    style::Style,
};
use git2::Repository;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use tl::Node::Tag;
use url::Position;
use url::Url;

use std::str::from_utf8;
use std::{
    convert::Infallible,
    fs::create_dir_all,
    net::SocketAddr,
    path::Path,
    process::{Child, Command},
};
use tokio::task::JoinHandle;
use viuer::{print_from_file, Config};

#[derive(Parser)]
#[command(name = "xtask")]
#[command(bin_name = "xtask")]
enum Cli {
    Crawl(CrawlArgs),
    Generate(GenerateArgs),
    FavIcon(FavIconArgs),
    Screenshot(ScreenshotArgs),
    Serve(ServeArgs),
    VisualDiff(VisualDiffArgs),
    Watch(WatchArgs),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct CrawlArgs {
    host: String,
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct FavIconArgs {
    page_path: String,
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct GenerateArgs {
    page_path: String,
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct ScreenshotArgs;

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct ServeArgs;

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct VisualDiffArgs;

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct WatchArgs;

fn run_generate() -> Child {
    Command::new("cargo")
        .arg("xtask")
        .arg("generate")
        .arg("pages")
        .spawn()
        .expect("failed to execute process")
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(hyper_staticfile::Static::new("_site")
        .serve(req)
        .await
        .unwrap())
}

fn serve() -> JoinHandle<()> {
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let serve = Server::bind(&SocketAddr::from(([127, 0, 0, 1], 62394))).serve(make_service);
    let local_addr = serve.local_addr();
    println!("Serving at {local_addr:?}");

    tokio::spawn(async move {
        if let Err(e) = serve.await {
            eprintln!("server error: {e}");
        }
    })
}

fn layout_for_page(factory: &Factory, mut page: Page) -> String {
    Layout {
        title: &factory.title,
        language: &factory.language,
        style: &factory.style.style(),
        description: &page.description(),
        body: &page.body(),
        page_title: None,
        footer: "",
        author: "",
    }
    .to_string()
}

async fn favicon_url(url: Url) -> Url {
    let cache = Cache::new("./cache");
    let body_bytes = cache.get(url.clone()).await.unwrap();
    let body = from_utf8(&body_bytes).unwrap();
    let dom = tl::parse(body, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let mut base_url: Url = url[..Position::BeforePath].parse().unwrap();
    let mut link_query = dom.query_selector(r#"link[rel~="icon"][href]"#).unwrap();

    if let Some(node_handle) = link_query.next() {
        if let Tag(tag) = node_handle.get(parser).unwrap() {
            url::Url::options()
                .base_url(Some(&base_url))
                .parse(
                    tag.attributes()
                        .get("href")
                        .unwrap()
                        .unwrap()
                        .try_as_utf8_str()
                        .unwrap(),
                )
                .unwrap()
        } else {
            base_url.path_segments_mut().unwrap().push("favicon.ico");
            base_url
        }
    } else {
        base_url.path_segments_mut().unwrap().push("favicon.ico");
        base_url
    }
}

async fn favicon(url: Url) {
    let cache = Cache::new("./favicon-cache");
    cache.get(favicon_url(url).await).await;
}

#[tokio::main]
async fn main() {
    match Cli::parse() {
        Cli::Crawl(CrawlArgs { host }) => {
            let mut sitemap = Output::new("./_site").sitemap().create();

            for mut url in Crawler::new() {
                url.set_host(Some(&host)).unwrap();
                url.set_port(None).unwrap();
                sitemap.push(&url);
            }
        }
        Cli::FavIcon(FavIconArgs { page_path }) => {
            let _output = Output::new("./_site");
            for page in Page::from_dir(&page_path).unwrap() {
                for url in page.link_urls() {
                    favicon(url).await;
                }
                // println!("{:?}", );

                // output
                //     .page(page.file_stem())
                //     .write_all(layout_for_page(&layout_factory.clone(), page).as_bytes())
                //     .unwrap();
            }
        }
        Cli::Generate(GenerateArgs { page_path: _ }) => {
            let output = Output::new("./_site");
            let style = Style::new(Path::new("style.css"));
            let layout_factory = Factory {
                style,
                title: "My Blog".to_string(),
                language: "en-AU".to_string(),
            };
            for page in Page::from_dir("./pages/").unwrap() {
                output
                    .page(page.file_stem())
                    .write_all(layout_for_page(&layout_factory.clone(), page).as_bytes())
                    .unwrap();

                // let output = Layout {
                //     body: &page.body(),
                //     footer: &History { commits: page.history() }.to_string(),
                //     author: page.author(),
                //     page_title: Some(&page.title()),
                //     description: &page.description(),
                // }.to_string();
                // page.output_path(Path::new("."), "html").display().to_string();
            }
        }
        Cli::Screenshot(_) => {
            let screenshots_dir = Path::new("./screenshots");
            create_dir_all(screenshots_dir).unwrap();

            let server_child = serve();
            let abort_handle = server_child.abort_handle();
            server_child.await.unwrap();
            let mut driver = chrome_driver::ChromeDriver::new().await;

            for url in Output::new("./_site").sitemap().open() {
                assert!(!abort_handle.is_finished());
                driver.goto(url.clone()).await;

                let path = url.path().strip_prefix('/').unwrap();
                let mut joined_path = screenshots_dir.join(path);
                joined_path = joined_path.with_extension("png");

                driver.screenshot(&joined_path).await;
            }

            abort_handle.abort();

            driver.quit().await;
        }
        Cli::Serve(_) => {
            serve().await.unwrap();
        }
        Cli::VisualDiff(_) => {
            let repo = match Repository::open(".") {
                Ok(repo) => repo,
                Err(e) => panic!("failed to open: {e}"),
            };
            let tree = repo.find_reference("HEAD").unwrap().peel_to_tree();
            for d in repo
                .diff_tree_to_workdir(Some(&tree.unwrap()), None)
                .unwrap()
                .deltas()
            {
                if d.old_file().path().unwrap().starts_with("screenshots/") {
                    let old_conf = Config {
                        x: 0,
                        y: 0,
                        width: Some(80),
                        height: Some(25),
                        ..Default::default()
                    };
                    print_from_file(d.old_file().path().unwrap(), &old_conf)
                        .expect("Image printing failed.");
                    panic!(
                        "changes in screenshot {:?} detected",
                        d.old_file().path().unwrap()
                    );
                }
            }
        }
        Cli::Watch(_) => {
            let mut child = run_generate();

            let mut watcher = recommended_watcher(move |res| {
                if let Ok(Event {
                    kind: _e @ (Modify(_) | Remove(_)),
                    paths,
                    ..
                }) = res
                {
                    for path in paths {
                        println!("{path:?}");
                    }
                    child.try_wait().unwrap().map_or((), |status| {
                        if status.success() {
                            println!("generating");
                            child = run_generate();
                        } else {
                            panic!()
                        }
                    });
                }
            })
            .unwrap();
            watcher
                .watch(Path::new("pages"), RecursiveMode::NonRecursive)
                .unwrap();
            watcher
                .watch(Path::new("src"), RecursiveMode::Recursive)
                .unwrap();
            watcher
                .watch(Path::new("Cargo.toml"), RecursiveMode::NonRecursive)
                .unwrap();
            watcher
                .watch(Path::new("Cargo.lock"), RecursiveMode::NonRecursive)
                .unwrap();

            serve().await.unwrap();
        }
    }
}
