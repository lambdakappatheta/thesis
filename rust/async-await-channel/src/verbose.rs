use async_std::{sync::channel, task};
use std::time::Duration;


pub async fn example() {
    let (s, r) = channel(1);
    s.send("Hello").await;

    task::spawn(async move {
        s.send("World").await;
    });

    task::sleep(Duration::from_secs(1)).await;

    println!("{}", r.recv().await.unwrap());
    println!("{}", r.recv().await.unwrap());
}


pub async fn example() {
    println!("a");
    let (s, r) = channel(1);
    println!("b");
    s.send("Hello").await;
    println!("c");

    task::spawn(async move {
        println!("d");
        s.send("World").await;
        println!("e");
    });

    println!("f");
    task::sleep(Duration::from_secs(5)).await;
    println!("g");

    println!("{}", r.recv().await.unwrap());
    println!("h");
    println!("{}", r.recv().await.unwrap());
}

/*
a
b
c
f
d
g
Hello
h
e
World
*/
