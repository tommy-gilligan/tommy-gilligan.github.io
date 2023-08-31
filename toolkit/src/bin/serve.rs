#[tokio::main]
async fn main() {
    println!("Listening on http://{}", toolkit::serve::run().await);
}
