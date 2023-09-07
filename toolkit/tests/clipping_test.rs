use git2::Repository;
use toolkit::crawl::Crawler;

#[tokio::test(flavor = "multi_thread")]
async fn clipping_test() {
    std::env::set_current_dir(Repository::open_from_env().unwrap().workdir().unwrap()).unwrap();
    let server = toolkit::serve::run(None).await;
    let mut browser = toolkit::browser::Browser::new(&server.1).await;
    for url in Crawler::new(&server.1).await {
        browser.goto(&url).await;

        let main = browser.main().await;
        let main_rect = main.rect().await.unwrap();
        let main_right = main_rect.x + main_rect.width;

        for child in browser.children().await {
            let child_rect = child.rect().await.unwrap();
            let child_right = child_rect.x + child_rect.width;
            assert!(
                child_right <= main_right,
                "{} intersects main",
                child.outer_html().await.unwrap()
            )
        }
    }
}
