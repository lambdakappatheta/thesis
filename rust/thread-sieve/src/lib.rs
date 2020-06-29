use std::{
    sync::mpsc::{sync_channel, Receiver, RecvError, SyncSender},
    thread,
};

fn generate(sender: SyncSender<usize>) {
    for i in 2.. {
        if sender.send(i).is_err() {
            break;
        }
    }
}

fn filter(prime: usize, receiver: Receiver<usize>, sender: SyncSender<usize>) {
    loop {
        let num = match receiver.recv() {
            Ok(msg) => msg,
            Err(RecvError) => break,
        };
        if num % prime != 0 {
            if sender.send(num).is_err() {
                break;
            }
        }
    }
}

pub fn sieve() {
    let (gen, mut end) = sync_channel(1);
    thread::spawn(move || generate(gen));

    for _ in 0..100 {
        let prime = match end.recv() {
            Ok(msg) => msg,
            Err(RecvError) => {
                break;
            }
        };
        println!("{}", prime);

        let (sender, receiver) = sync_channel(1);
        thread::spawn(move || filter(prime, end, sender));
        end = receiver;
    }
}
