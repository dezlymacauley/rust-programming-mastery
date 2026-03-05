/*
    ABOUT: Receiving messages from multiple channels

*/

use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread::{sleep, spawn, JoinHandle};

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
            String::from("🗃️ dp_thread: Connecting to server..."),
            String::from("🗃️ dp_thread: Fetching data..."),
            String::from("🗃️ dp_thread: Processing data..."),
            String::from("🗃️ dp_thread: Task complete."),
        ];

        for log in data_processing_logs {
            // Imagine each log message takes 2 seconds to be generated
            sleep(Duration::from_secs(2));
            tx1.send(log).unwrap();
        }
    });

    let profile_loading_thread: JoinHandle<()> = spawn(move || {
        let profile_loading_logs = vec![
            String::from("👤 pl_thread: Loading user profile..."),
            String::from("👤 pl_thread: Fetching profile picture..."),
            String::from("👤 pl_thread: Applying user settings..."),
            String::from("👤 pl_thread: Profile ready."),
        ];

        for log in profile_loading_logs {
            // Imagine each log message takes 1 second to be generated
            sleep(Duration::from_secs(1));
            tx2.send(log).unwrap();
        }
    });

    //_________________________________________________________________________
   
    // SECTION: Receiving Messages
    
    //_________________________________________________________________________

    // TIP: Method 1 => Just use a for loop,
    // this is the most idiomatic way in Rust

    for status_update in rx {
        println!("{status_update}");
    }
    
    //_________________________________________________________________________

    // TIP: Method 2 => This is the equivalent way to do this using
    // an infinite loop, with match and break

    // loop {
    //     match rx.recv() {
    //         Ok(status_update) => println!("{status_update}"),
    //         Err(_) => break
    //     }
    // }
    
    //_________________________________________________________________________


    // Remember to use the JoinHandle of both threads to ensure that the
    // thread `fn main()` does not exit before the data_processing_thread
    // and the profile_loading_thread have completed
    // all their respective tasks.
    data_processing_thread.join().unwrap();
    profile_loading_thread.join().unwrap();
}

/*
    NOTE: This should be the output

    👤 pl_thread: Loading user profile...
    🗃️ dp_thread: Connecting to server...
    👤 pl_thread: Fetching profile picture...
    👤 pl_thread: Applying user settings...
    🗃️ dp_thread: Fetching data...
    👤 pl_thread: Profile ready.
    🗃️ dp_thread: Processing data...
    🗃️ dp_thread: Task complete.

*/
