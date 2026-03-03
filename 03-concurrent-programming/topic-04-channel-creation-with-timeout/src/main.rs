/*
    ABOUT: Creating Channels - Non-blocking Example

    rx.recv_timeout

    RecvTimeoutError

*/

use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};

use std::thread::{sleep, spawn};

use std::time::Duration;

fn main() {
    //_________________________________________________________________________

    // STEP: 1 => Create the channel

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    //_________________________________________________________________________

    // STEP: 2 => Create a thread

    let loading_player_rankings_thread = spawn(move || {
        tx.send("Loading player ranking data...".to_string())
            .unwrap();

        tx.send("25% done".to_string()).unwrap();
        sleep(Duration::from_secs(3));

        tx.send("50% done".to_string()).unwrap();
        sleep(Duration::from_secs(3));

        tx.send("75% done".to_string()).unwrap();
        sleep(Duration::from_secs(3));

        tx.send("The loading_player_rankings_thread is done.".to_string())
            .unwrap();
    });

    //_________________________________________________________________________

    // STEP: 5 => loop with a timeout

    // Non-blocking-ish loop using `recv_timeout`
    // The main thread waits up to 2 seconds for a message.
    // If no message arrives within that time, the loop can do other work
    // before trying to receive again.
    loop {
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(status_message) => {
                println!("status_message: {status_message}");
            }
            Err(RecvTimeoutError::Timeout) => {
                println!("Main thread doing other work while waiting...");
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    //_________________________________________________________________________

    // STEP: 4 => Add the JoinHandle to ensure that the main thread does not
    // exit before the loading_player_rankings_thread has finished as well.

    loading_player_rankings_thread.join().unwrap();

    //_________________________________________________________________________

    println!("The main thread is done");
}

// NOTE: This should be the output

// status_message: Loading player ranking data...
// status_message: 25% done
// Main thread doing other work while waiting...
// status_message: 50% done
// Main thread doing other work while waiting...
// status_message: 75% done
// Main thread doing other work while waiting...
// status_message: The loading_player_rankings_thread is done.
// The main thread is done
