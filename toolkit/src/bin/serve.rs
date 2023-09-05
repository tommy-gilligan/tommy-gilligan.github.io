#[tokio::main]
async fn main() {
    let address = toolkit::serve::run(None).await;
    println!("Listening on http://{}", address.1);
    address.0.await.unwrap();
}
