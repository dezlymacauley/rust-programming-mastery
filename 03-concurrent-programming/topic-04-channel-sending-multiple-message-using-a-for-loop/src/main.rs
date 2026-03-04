/*
    ABOUT: Sending mutliple messages using a for loop

*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Simulate log messages being produced over time
        let logs = vec![
            String::from("Connecting to server..."),
            String::from("Fetching data..."),
            String::from("Processing data..."),
            String::from("Task complete."),
        ];

        for log in logs {
            tx.send(log).unwrap();          // Send each message as it's ready
            thread::sleep(Duration::from_secs(1)); // Simulate time delay
        }
    });

    // Main thread receives messages as they come
    for received in rx {
        println!("Got: {received}");
    }
}
