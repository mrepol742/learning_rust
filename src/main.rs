use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 10;

    // }

    // println!("m = {:?}", m);

    let mut num = m.lock().unwrap();
    *num = 10;
    drop(num);

    let mut num1 = m.lock().unwrap();
    *num1 = 10;
    drop(num1);
}
