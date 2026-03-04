/*
    ABOUT: try_recv
*/

use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread::{spawn, sleep};

use std::time::Duration;

fn main() {
    //_________________________________________________________________________

    // STEP: 1 => Create a channel

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    //_________________________________________________________________________

    // STEP: 2 => Create a thread

    let file_download_thread = spawn(move || {

        tx.send("Starting download...".to_string()).unwrap();

        sleep(Duration::from_secs(4));

        tx.send("Download completed.".to_string()).unwrap();
    });

    //_________________________________________________________________________
    
    // STEP: 3 => Use the reciever to receive the messages from the channel 

    // Attempt 1
    match rx.try_recv() {
        Ok(status_update) => {
            println!("{status_update}");
        },
        Err(error) => {
            println!("There is no status update available: {error}");
        }
    }

    println!("The main thread continues while waiting for the download");
    
    // Attempt 2
    match rx.try_recv() {
        Ok(status_update) => {
            println!("{status_update}");
        },
        Err(error) => {
            println!("There is no status update available: {error}");
        }
    }
    

    //_________________________________________________________________________
    
    // STEP: 4 => Join thread

    file_download_thread.join().unwrap();

    //_________________________________________________________________________
}
