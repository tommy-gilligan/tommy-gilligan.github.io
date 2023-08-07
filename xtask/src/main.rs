#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use clap::Parser;
use generation::chrome_driver;
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
use sitemap::{
    reader::{SiteMapEntity, SiteMapReader},
    structs::{Location, UrlEntry},
    writer::SiteMapWriter,
};
use std::{
    convert::Infallible,
    ffi::OsStr,
    fs::{create_dir_all, read_dir, File},
    net::SocketAddr,
    path::{Path, PathBuf},
    process::{Child, Command},
};
use tokio::task::JoinHandle;
use url::Url;
use viuer::{print_from_file, Config};

#[derive(Parser)]
#[command(name = "xtask")]
#[command(bin_name = "xtask")]
enum Cli {
    Crawl(CrawlArgs),
    Generate(GenerateArgs),
    Screenshot(ScreenshotArgs),
    Serve(ServeArgs),
    VisualDiff(VisualDiffArgs),
    Watch(WatchArgs),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct CrawlArgs {
    base_url: String,
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

const EXTENSION: &str = "html";

pub fn from_dir(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    Ok(read_dir(path)?
        .filter_map(|e| {
            e.map_or(None, |f| match f.file_type() {
                Ok(file_type) if file_type.is_file() => match f.path().extension() {
                    Some(extension) if extension == OsStr::new(EXTENSION) => Some(f.path()),
                    _ => None,
                },
                _ => None,
            })
        })
        .collect())
}

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

#[tokio::main]
async fn main() {
    match Cli::parse() {
        Cli::Crawl(CrawlArgs { base_url }) => {
            let base_url: Url = base_url.parse().unwrap();
            assert_eq!(base_url.scheme(), "https");
            let base_url: Url = base_url[..url::Position::BeforePath].parse().unwrap();

            let output_dir = Path::new("./_site");
            let mut output = File::create(output_dir.clone().join("sitemap.xml")).unwrap();
            let sitemap_writer = SiteMapWriter::new(&mut output);
            let mut urlwriter = sitemap_writer
                .start_urlset()
                .expect("Unable to write urlset");

            for file in from_dir(output_dir).unwrap() {
                let mut url = base_url.clone();
                url.set_path(file.file_name().unwrap().to_str().unwrap());
                urlwriter
                    .url(UrlEntry::builder().loc(url.to_string()))
                    .expect("Unable to write url");
            }

            urlwriter.end().expect("Unable to write close tags");
        }
        Cli::Generate(GenerateArgs { page_path: _ }) => {
            generation::generate::main();
            println!("generating");
        }
        Cli::Screenshot(_) => {
            let output_dir = Path::new("./_site");
            create_dir_all(output_dir).unwrap();

            let screenshots_dir = Path::new("./screenshots");
            create_dir_all(screenshots_dir).unwrap();

            let server_child = serve();
            let abort_handle = server_child.abort_handle();
            server_child.await.unwrap();
            let file =
                File::open(output_dir.clone().join("sitemap.xml")).expect("Unable to open file.");
            let parser = SiteMapReader::new(file);

            let mut driver = chrome_driver::ChromeDriver::new().await;
            for entity in parser {
                if let SiteMapEntity::Url(UrlEntry {
                    loc: Location::Url(url),
                    ..
                }) = entity
                {
                    assert!(!abort_handle.is_finished());
                    driver.goto(url.clone()).await;

                    let path = url.path().strip_prefix('/').unwrap();
                    let mut joined_path = screenshots_dir.join(path);
                    joined_path = joined_path.with_extension("png");

                    driver.screenshot(&joined_path).await;
                }
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
