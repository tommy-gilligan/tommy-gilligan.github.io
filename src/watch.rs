#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use notify::{Event, EventKind::{Remove, Modify}};


use std::{
    path::Path,
};
use notify::{Watcher, RecursiveMode};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr};

async fn handle(req: Request<Body>) -> std::result::Result<Response<Body>, Infallible> {
    Ok(hyper_staticfile::Static::new("_site")
        .serve(req)
        .await
        .unwrap())
}

use std::process::Command;

const PATH: &str = "target/debug/generate";

#[tokio::main]
async fn main() {
    let mut child = Command::new(PATH).spawn().expect("failed to execute process");
    let mut watcher = notify::recommended_watcher(move |res| {
      if let Ok(Event { kind: e @ (Modify(_) | Remove(_)), .. }) = res {
          println!("{:?}", e);
          child.try_wait().unwrap().map_or((), |status| if status.success() {
                      println!("generating");
                      child = Command::new(PATH).spawn().expect("failed to execute process");
                  } else {
                      panic!()
                  });
      }
    }).unwrap();
    watcher.watch(Path::new("pages"), RecursiveMode::NonRecursive).unwrap();
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let serve = Server::bind(&SocketAddr::from(([127, 0, 0, 1], 62394))).serve(make_service);
    let local_addr = serve.local_addr();
    println!("Serving at {local_addr:?}");

    tokio::spawn(async move {
        if let Err(e) = serve.await {
            eprintln!("server error: {e}");
        }
    })
    .await
    .unwrap();
}
