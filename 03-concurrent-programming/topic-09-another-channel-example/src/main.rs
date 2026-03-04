use std::thread::{JoinHandle, sleep, spawn};

use std::time::Duration;

fn main() {
    let task_a_thread: JoinHandle<()> = spawn(|| {
        // Imagine it task 1 seconds to start Task A
        sleep(Duration::from_secs(1));
        println!("Starting Task A...");

        // Imagine it task 2 seconds to start Task A
        sleep(Duration::from_secs(2));
        println!("Task A completed");
    });

    println!("Task B completed");
    println!("Task C completed");

    task_a_thread.join().unwrap();

    println!("Task D completed");

}
