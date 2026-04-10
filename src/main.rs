use std::thread;
use std::time::Duration;

fn main() {
    let job_1 = thread::spawn(|| {
        println!("Job 1 is starting.");
        println!("waiting for Job 2 to complete");
        // thread::park_timeout(Duration::from_secs(2));
        // thread::sleep(Duration::from_secs(2));
        thread::yield_now();

        println!("Job 1 is resuming.");
        println!("Job 1 is completed.");
    });

    let job_2 = thread::spawn(|| {
        println!("Job 2 is starting.");
        println!("Job 2 is finished");
    });

    job_2.join().unwrap();

    println!("Job 2 is now complete");
    println!("Job 1 will now resume");

    job_1.thread().unpark();
    job_1.join().unwrap();
}
