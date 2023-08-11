#![feature(async_closure)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use clap::Parser;
use generation::{chrome_driver, crawl::Crawler, output::Output};
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
        .arg("run")
        .arg("--bin")
        .arg("generate")
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
                    paths: _,
                    ..
                }) = res
                {
                    child.try_wait().unwrap().map_or((), |status| {
                        if status.success() {
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
