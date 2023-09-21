use futures::sink::SinkExt;

use futures_util::future;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response,
};
use hyper_staticfile::Static;
use std::{io::Error as IoError, path::Path};

#[allow(clippy::future_not_send)]
async fn handle_request<B>(
    mut req: Request<B>,
    static_: Static,
    mut refresh_channel: Option<tokio::sync::watch::Receiver<()>>,
) -> Result<Response<Body>, IoError> {
    if hyper_tungstenite::is_upgrade_request(&req) {
        let (response, websocket) = hyper_tungstenite::upgrade(&mut req, None).unwrap();

        tokio::spawn(async move {
            let mut websocket = websocket.await.unwrap();
            let channel = refresh_channel.as_mut().unwrap();
            channel.changed().await.unwrap();
            while channel.changed().await.is_ok() {
                websocket.send("refresh".into()).await.unwrap();
            }
        });

        Ok(response)
    } else {
        static_.clone().serve(req).await
    }
}

#[allow(clippy::unused_async)]
pub async fn run(
    refresh_channel: Option<tokio::sync::watch::Receiver<()>>,
) -> (tokio::task::JoinHandle<()>, std::net::SocketAddr) {
    let static_ = Static::new(Path::new(crate::SITE));
    let make_service = make_service_fn(move |_| {
        let static_ = static_.clone();
        let refresh_channel = refresh_channel.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| {
            let refresh_channel = refresh_channel.clone();
            handle_request(req, static_.clone(), refresh_channel)
        }))
    });

    let server = hyper::Server::bind(
        &std::net::SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, 3000).into(),
    )
    .serve(make_service);

    let addr = server.local_addr();
    (tokio::spawn(async { server.await.unwrap() }), addr)
}
