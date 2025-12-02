// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let t = thread::spawn(move || {
//         let my_vec = vec![1,2,3,4,5,6];
//         for i in my_vec {
//             tx.send(i).unwrap();
//         }
//     });

//     // for received_vals in rx {
//     //     println!("I received the value of {}", received_vals);
//     // }
//     //
//     let received_vals_vec = rx.iter().collect::<Vec<i32>>();
//     println!("The received values are {:?}", received_vals_vec);
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let my_vec = vec![1,2,3,4,5,6];
//         for i in my_vec {
//             tx.send(i).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let my_vec = vec![7,8,9,0,10,11];
//         for i in my_vec {
//             tx1.send(i).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received_vals in rx {
//         println!("I received the value of {}", received_vals);
//     }
// }

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn timer(d: i32, tx: mpsc::Sender<i32>){
    thread::spawn(move || {
        println!("{} send!", d);
        tx.send(d).unwrap();
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        timer(i, tx.clone());
    }

    drop(tx);

    for receiving_vals in rx {
        println!("{} received", receiving_vals);
    }
}
