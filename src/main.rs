use std::thread;

fn main() {
    let mut vec = vec![1, 2, 3, 4];
    let mut x = 0;

    // thread::scope(|some_scope| {
        thread::spawn(|| {
            println!("I am the first thread in the scope");
            println!("{:?}", vec);
        });

        thread::spawn(|| {
            println!("I am the second thread in the scope");
            x += 45;
        });
    // });

    println!("The threads are now complete");
    vec.push(5);
    println!("x: {:?} and a: {:?}", x, vec);
}
