use reqwest::Client;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    if let Err(e) = tokio::spawn(async move {
        task1().await;
    })
    .await
    {
        println!("{:?}", e);
    };
    // loop {
    //     println!("start task");
    //     if let Err(e) = tokio::spawn(async move {
    //         request().await;
    //     })
    //     .await {
    //         println!("{:?}", e);
    //     }
    //     println!("end task")ƒ;
    // }
}

async fn task1() {
    let mut interval: time::Interval = time::interval(Duration::from_secs(10));
    if let Err(e) = tokio::spawn(async move {
        loop {
            interval.tick().await;
            println!("start task");
            request().await;
            println!("end task");
            tokio::task::yield_now().await;
        }
    }).await {
        println!("{:?}", e);
    }
}

async fn request() {
    let mut ks = vec![];
    for _i in 0..100 {
        ks.push(tokio::spawn(async move {
            let url = "https://api.bulgarianhistory.shop";
            let client = Client::builder().timeout(Duration::from_secs(60)).build();
            match client {
                Ok(c) => {
                    if let Err(e) = c.get(url).send().await {
                        println!("{:?}", e);
                    };
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
            tokio::task::yield_now().await;
        }));
    }

    for k in ks {
        if let Err(e) = k.await {
            println!("{:?}", e);
        };
    }
}
