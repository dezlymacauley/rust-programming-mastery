/*
    ABOUT: recv

*/

use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread::{sleep, spawn};

use std::time::Duration;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    let api_data_fetch_thread = spawn(move || {
        tx.send("Fetching API data...".to_string()).unwrap();
        
        sleep(Duration::from_secs(2));

        tx.send("API data successfully fetched".to_string())
            .unwrap();
    });

    // rx.recv() can be used to receive multiple message
    // but it only receives one messsage at a time

    //_________________________________________________________________________

    // SECTION: Method 1 - Handle the recieval of each message

    // let first_status_message = rx.recv().unwrap();
    // println!("{first_status_message}");

    // let second_status_messsage = rx.recv().unwrap();
    // println!("{second_status_messsage}");

    //_________________________________________________________________________

    // SECTION: Method 2 - Use a for loop

    for status_message in rx {
        println!("{status_message}")
    }

    //_________________________________________________________________________

    api_data_fetch_thread.join().unwrap();
}
