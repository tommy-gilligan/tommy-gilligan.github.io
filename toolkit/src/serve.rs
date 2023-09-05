use futures_util::future;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response,
};
use hyper_staticfile::Static;
use std::{io::Error as IoError, path::Path};

async fn handle_request<B>(req: Request<B>, static_: Static) -> Result<Response<Body>, IoError> {
    static_.clone().serve(req).await
}

pub async fn run(
    _refresh_channel: Option<tokio::sync::mpsc::Receiver<bool>>,
) -> (tokio::task::JoinHandle<()>, std::net::SocketAddr) {
    let static_ = Static::new(Path::new(crate::SITE));
    let make_service = make_service_fn(move |_| {
        let static_ = static_.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, static_.clone())))
    });

    let server =
        hyper::Server::bind(&std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0).into())
            .serve(make_service);

    let addr = server.local_addr();
    (tokio::spawn(async { server.await.unwrap() }), addr)
}
