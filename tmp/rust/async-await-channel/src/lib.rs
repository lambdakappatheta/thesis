use async_std::{sync::channel, task};
use std::time::Duration;

pub fn example() -> task::JoinHandle<()> {
    task::spawn(async move {
        let (s, r) = channel(1);
        s.send("Hello").await;

        task::spawn(async move {
            s.send("World").await;
        });

        task::sleep(Duration::from_secs(1)).await;

        println!("{}", r.recv().await.unwrap());
        println!("{}", r.recv().await.unwrap());
    })
}
