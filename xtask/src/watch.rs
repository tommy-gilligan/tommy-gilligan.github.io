use generation::tokiort::TokioIo;
use notify::{
    recommended_watcher, Event,
    EventKind::{Modify, Remove},
    RecursiveMode, Watcher,
};
use std::net::IpAddr;
use std::net::SocketAddr;
use std::path::Path;
use tokio::net::TcpListener;
use url::Url;

// TODO: config should be a superset of serve config
// TODO: what to watch configurable recompile and regenerate vs just regenerate
#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "articles")]
    pub articles: String,
    #[arg(short, long, default_value = "_site")]
    pub output: String,
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
}

pub async fn watch(config: &Args) {
    let ip: IpAddr = config.host.parse().unwrap();
    let addr: SocketAddr = (ip, config.port).into();
    let listener = TcpListener::bind(addr).await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    let base_url: Url = format!("http://{local_addr}").parse().unwrap();

    let output = config.output.clone();
    let mut child = crate::generate::run(&base_url);
    let mut watcher = recommended_watcher(move |res| {
        if let Ok(Event {
            kind: _e @ (Modify(_) | Remove(_)),
            ..
        }) = res
        {
            match child.try_wait() {
                Ok(Some(status)) if status.success() => {
                    println!("regenerating");
                    child = crate::generate::run(&base_url);
                }
                _ => (),
            }
        }
    })
    .unwrap();
    watcher
        .watch(Path::new(&config.articles), RecursiveMode::NonRecursive)
        .unwrap();

    println!("Serving at http://{}", listener.local_addr().unwrap());
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        let service = generation::serve::Service::new(output.clone().into());
        tokio::task::spawn(async move {
            hyper::server::conn::http1::Builder::new()
                .serve_connection(io, service)
                .await
                .unwrap();
        });
    }
}
