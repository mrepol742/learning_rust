use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    // let rx1 = rx;
    let t = thread::spawn(move || {
        let val = String::from("melvin");
        println!("Value sending from the thread");
        tx.send(val).unwrap();
        // println!("This may execute after the statement in the main");
        // println!("Val is {:?}", val);
    });

    // let received = rx.recv().unwrap();
    // println!("Received: {:?}", received);

    t.join();

    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("received value {:?}", received_value);
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}
