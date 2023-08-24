use clap::Parser;
use generation::tokiort::TokioIo;

use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value = "_site")]
    pub output: String,
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
}

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let ip: IpAddr = config.host.parse().unwrap();
    let addr: SocketAddr = (ip, config.port).into();

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        let service = generation::serve::Service::new(config.output.clone().into());
        tokio::task::spawn(async move {
            hyper::server::conn::http1::Builder::new()
                .serve_connection(io, service)
                .await
                .unwrap();
        });
    }
}
