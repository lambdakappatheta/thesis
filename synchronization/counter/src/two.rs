fn count() {
    let mut n = 0;

    let handle1 = std::thread::spawn(|| {
        for _ in 1..100 {
            n += 1;
        }
    });
    let handle2 = std::thread::spawn(|| {
        for _ in 1..100 {
            n -= 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", n);
}
