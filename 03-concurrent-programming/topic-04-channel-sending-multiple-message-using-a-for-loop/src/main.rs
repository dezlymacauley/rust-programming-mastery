/*
    ABOUT: Sending mutliple messages using a for loop

*/

use std::sync::mpsc::{channel, Sender, Receiver};

use std::thread::{JoinHandle, sleep, spawn};

use std::time::Duration;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    let data_processing_thread: JoinHandle<()> = spawn(move || {
        let task_logs = vec![
            String::from("Connecting to server..."),
            String::from("Fetching data..."),
            String::from("Processing data..."),
            String::from("Task complete."),
        ];

        for log in task_logs {
            // Send a message after each second
            sleep(Duration::from_secs(1));
            tx.send(log).unwrap();
        }
    });

    for status_update in rx {
        println!("{status_update}");
    }

    data_processing_thread.join().unwrap();
}
