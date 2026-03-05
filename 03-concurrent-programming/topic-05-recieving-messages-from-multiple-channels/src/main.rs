/*
    ABOUT: Receiving messages from multiple channels

*/

use std::sync::mpsc::{channel, Sender, Receiver};

use std::thread::{JoinHandle, sleep, spawn};

use std::time::Duration;

fn main() {

    // `tx1` stands for `transmitter 1`
    // This will be used by the data_processing_thread to send messages to
    // `rx` which is the receiver.
    // You only need one `rx` because `rx` can receive messages 
    // from multiple senders (aka multiple producers, aka multiple threads)
    let (tx1, rx): (Sender<String>, Receiver<String>) = channel();

    // `tx2` stands for `transmitter 2`
    // This will be used by the profile_loading_threading to send messages to
    // `rx` which is the receiver.

    // TIP: If you need to clone a transmitter, clone it before it used.
    // Once the data_processing_thread, takes ownership of the transmitter,
    // you will not be able to clone that transmitter anymore.
    
    let tx2 = tx1.clone();
    
    let data_processing_thread: JoinHandle<()> = spawn(move || {
        let data_processing_logs = vec![
            String::from("Connecting to server..."),
            String::from("Fetching data..."),
            String::from("Processing data..."),
            String::from("Task complete."),
        ];

        for log in data_processing_logs {

            // Imagine each log message takes 2 seconds to be generated
            sleep(Duration::from_secs(2));
            tx1.send(log).unwrap();
        }
    });
    

    let profile_loading_thread: JoinHandle<()> = spawn(move || {
        let profile_loading_logs = vec![
            String::from("Loading user profile..."),
            String::from("Fetching profile picture..."),
            String::from("Applying user settings..."),
            String::from("Profile ready."),
        ];

        for log in profile_loading_logs {
            // Imagine each log message takes 1 second to be generated
            sleep(Duration::from_secs(1));
            tx2.send(log).unwrap();
        }
    });

    for status_update in rx {
        println!("{status_update}");
    }

    // Remember to use the JoinHandle of both threads to ensure that the
    // thread `fn main()` does not exit before the data_processing_thread
    // and the profile_loading_thread have completed 
    // all their respective tasks.
    data_processing_thread.join().unwrap();
    profile_loading_thread.join().unwrap();
}

/*
    NOTE: This should be the output

    Loading user profile...
    Connecting to server...
    Fetching profile picture...
    Applying user settings...
    Fetching data...
    Profile ready.
    Processing data...
    Task complete.

*/
