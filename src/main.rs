use reqwest::Client;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut interval: time::Interval = time::interval(Duration::from_secs(5));
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            println!("start task");
            request().await;
            println!("end task");
        }
    })
    .await
    .unwrap();
}

async fn request() {
    for _ in 0..100 {
        tokio::spawn(async move {
            let url = "https://tokio.rs";
            let client = Client::builder()
                .timeout(Duration::from_secs(300))
                .build()
                .unwrap();

            client.get(url).send().await.unwrap().text().await.unwrap();
        });
    }
}
