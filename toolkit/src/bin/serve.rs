#[tokio::main]
async fn main() {
    let address = toolkit::serve::run().await;
    println!("Listening on http://{}", address.1);
    address.0.await.unwrap();
}
