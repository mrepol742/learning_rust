use std::thread;
use std::time::Duration;

fn main() {
    println!("This will be printed");

    let t = thread::spawn(|| {
        println!("Hello 1");
        println!("Hello 2");
        println!("Hello 3");
        println!("Hello 4");
        println!("Hello 5");
        println!("Hello 6");
    });

    thread::sleep(Duration::from_millis(1));
    println!("Hello 4 main");
    println!("Hello 5 main");
    println!("Hello 6 main");

    t.join();
}
