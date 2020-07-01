/*
fn main() {
    let mut n = 0;
    let r = &mut n;

    let h = std::thread::spawn(move || {
        for _ in 1..100 {
            *r += 1;
        }
    });

    h.join().unwrap();

    //println!("{}", n);
}
*/


fn main() {
    let mut n = 0;
    let s;
    unsafe {
        s = std::mem::transmute::<&mut i32, &'static mut i32>(&mut n);
    };

    let h = std::thread::spawn(move || {
        for _ in 1..100 {
            *s += 1;
        }
    });

    h.join().unwrap();

    println!("{}", n);
}


/*
fn main() {
    counter::reference::count();
}
*/

/*
fn main() {
    let handle = std::thread::spawn(|| fun(1));
    handle.join().unwrap();
}

fn fun(n: i32) {
    println!("{}", n);
}
*/
