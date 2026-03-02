/*
    ABOUT: Spawning and joining a thread

*/

use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    println!("File A received");

    let file_b_thread = spawn(|| {
        sleep(Duration::from_secs(2));
        println!("File B received");
    });

    println!("File C received");

    // This will block the main thread from completing,
    // until the file_b_thread has completed.
    file_b_thread.join().unwrap();
}
