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

pub async fn sieve(n: usize) -> Vec<usize> {
    let mut primes = vec![];
    let (s, mut r) = channel(1);
    task::spawn(generate(s));

    for _ in 0..n {
        let prime = match r.recv().await {
            Ok(msg) => msg,
            Err(_) => break,
        };
        primes.push(prime);

        let (s1, r1) = channel(1);
        task::spawn(filter(prime, r, s1));
        r = r1;
    }
    primes
}
