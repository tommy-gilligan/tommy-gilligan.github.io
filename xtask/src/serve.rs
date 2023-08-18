use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::sync::OnceLock;
use std::{convert::Infallible, net::SocketAddr};
use tokio::task::JoinHandle;

static OUTPUT: OnceLock<String> = OnceLock::new();

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "_site")]
    pub output: String,
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,
    #[arg(short, long, default_value = "127.0.0.1")]
    pub host: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            output: "_site".to_string(),
            port: 0,
            host: "127.0.0.1".to_string(),
        }
    }
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(hyper_staticfile::Static::new(OUTPUT.get().unwrap())
        .serve(req)
        .await
        .unwrap())
}

pub fn serve(config: &Args) -> JoinHandle<()> {
    OUTPUT.set(config.output.clone()).unwrap();

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let ip: std::net::IpAddr = config.host.parse().unwrap();
    let serve = Server::bind(&SocketAddr::from((ip, config.port))).serve(make_service);
    let local_addr = serve.local_addr();
    println!("Serving at {local_addr:?}");

    tokio::spawn(async move {
        if let Err(e) = serve.await {
            eprintln!("server error: {e}");
        }
    })
}
