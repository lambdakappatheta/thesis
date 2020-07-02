pub fn count() {
    let mut n = 0;
    let s;
    unsafe {
        s = std::mem::transmute::<&mut i32, &'static mut i32>(&mut n);
    };

    let handle1 = std::thread::spawn(move || {
        for _ in 1..100 {
            *s += 1;
        }
    });

    handle1.join().unwrap();

    println!("{}", n);
}
