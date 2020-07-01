/*
use std::thread;

pub fn count() {
    let mut n = 0;
    let h1 = thread::spawn(move || increment(&mut n));
    let h2 = thread::spawn(move || decrement(&mut n));

    h1.join().unwrap();
    h2.join().unwrap();

    println!("{}", n);
}

fn increment(n: &mut i32) {
    for _ in 1..100 {
        println!("inc");
        *n += 1;
        println!("{}", *n);
    }
}

fn decrement(n: &mut i32) {
    for _ in 1..100 {
        *n -= 1;
    }
}
*/

/*
use std::thread;

pub fn count() {
    let mut n = 0;
    let r1 = &mut n;
    let r2 = &mut n;
    let h1 = thread::spawn(move || increment(r1));
    let h2 = thread::spawn(move || decrement(r2));

    h1.join().unwrap();
    h2.join().unwrap();

    println!("{}", n);
}

fn increment(n: &mut i32) {
    for _ in 1..100 {
        println!("inc");
        *n += 1;
        println!("{}", *n);
    }
}

fn decrement(n: &mut i32) {
    for _ in 1..100 {
        *n -= 1;
    }
}
*/

pub fn count() {
    let mut n = 0;
    // n is Copy
    let h1 = std::thread::spawn(move || {
        for _ in 1..100 {
            n += 1;
        }
    });
    // n is Copy
    let h2 = std::thread::spawn(move || {
        for _ in 1..100 {
            n += 1;
        }
    });
    h1.join().unwrap();
    h2.join().unwrap();

    println!("{}", n);
}

