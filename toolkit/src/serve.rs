use hyper::{body::Incoming as IncomingBody, Request, Response};
use hyper_staticfile::Static;

use crate::tokiort::TokioIo;
use std::{future::Future, path::PathBuf, pin::Pin};
use tokio::net::TcpListener;

pub struct Service {
    inner: Static,
}

impl Service {
    #[must_use]
    pub fn new(output: PathBuf) -> Self {
        Self {
            inner: hyper_staticfile::Static::new(output),
        }
    }
}

impl hyper::service::Service<Request<IncomingBody>> for Service {
    type Response = Response<hyper_staticfile::Body>;
    type Error = std::io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        Box::pin(self.inner.clone().serve(req))
    }
}

pub async fn run() -> (tokio::task::JoinHandle<()>, std::net::SocketAddr) {
    let listener = TcpListener::bind(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::LOCALHOST,
        0,
    ))
    .await
    .unwrap();
    let local_addr = listener.local_addr().unwrap();

    (
        tokio::task::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                let io = TokioIo::new(stream);

                let service = Service::new(crate::SITE.into());
                tokio::task::spawn(async move {
                    match hyper::server::conn::http1::Builder::new()
                        .serve_connection(io, service)
                        .await
                    {
                        Ok(_) => (),
                        Err(e) => println!("{e}"),
                    }
                });
            }
        }),
        local_addr,
    )
}
