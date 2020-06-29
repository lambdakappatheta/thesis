use async_std::{
    sync::{channel, Receiver, Sender},
    task,
};

async fn generate(sender: Sender<usize>) {
    for i in 2.. {
        sender.send(i).await;
    }
}

async fn filter(prime: usize, receiver: Receiver<usize>, sender: Sender<usize>) {
    loop {
        let num = match receiver.recv().await {
            Ok(msg) => msg,
            Err(_) => break,
        };
        if num % prime != 0 {
            if num % prime != 0 {
                sender.send(num).await;
            }
        }
    }
}

pub async fn sieve() {
    let (gen, mut end) = channel(1);
    task::spawn(generate(gen));

    for _ in 0..100 {
        let prime = match end.recv().await {
            Ok(msg) => msg,
            Err(_) => break,
        };
        println!("{}", prime);

        let (sender, receiver) = channel(1);
        task::spawn(filter(prime, end, sender));
        end = receiver;
    }
}
