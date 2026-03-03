/*
    ABOUT: Creating Channels

    A channel is a communication device,
    that allows threads to send and receive messages from each other.

    A JoinHandle is great because it blocks the main thread `fn main()`,
    from exiting before another thread has finished
    its task before the program ends.

    A JoinHandle is great for ensuring that you get some kind of response
    from a thread when that thread completes its finally task.

    However in many situations you may want to recieve some kind
    of status updates or progress updates from a thread
    as it completes its task.

*/

// mpsc stands for "Multiple producers, single consumer"
// In simple terms, multiple senders with one receiver.
use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread::{sleep, spawn};

use std::time::Duration;

fn main() {
    //_________________________________________________________________________

    // STEP: 1 => Create the channel

    // Each channel consists of two parts:
    // 1. A `sender` variable, which is usually named tx (transmitter)
    // 2. A `receiver` variable, which is usually named rx (reciever)
    //
    // The sender variable (tx) allows a sender
    // to send a message to the channel.
    // The sender variable can be cloned if you want multiple senders
    // (aka producers) to be able to send messages to the channel
    //
    // The receiver variable (rx) waits for messages to be sent through
    // the channel. This is refered to as consuming the message.
    // There can only be one reciever variable (rx).
    //
    // So the `mpsc` module stands for `multiple producers (senders)`,
    // `single consumer (receiver)`
    // use std::sync::mpsc::channel;
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    //_________________________________________________________________________

    // STEP: 2 => Create a thread

    // A thread to load the latest rankings of players in a video game,
    // while the rest of the game is playable.

    // TIP: Don't forget the `move` keyword. The closure should take control
    // of the the sender variable, to ensure that the sender variable lives
    // for as long as the thread is valid.

    let loading_player_rankings_thread = spawn(move || {
        tx.send("Loading player ranking data...".to_string()).unwrap();

        tx.send("25% done".to_string()).unwrap();
        sleep(Duration::from_secs(1));

        tx.send("50% done".to_string()).unwrap();
        sleep(Duration::from_secs(1));

        tx.send("75% done".to_string()).unwrap();
        sleep(Duration::from_secs(1));

        tx.send("The loading_player_rankings_thread is done.".to_string()).unwrap();
    });
    
    //_________________________________________________________________________

    // STEP: 5 => Add the reciever

    // You can think of rx as a list of Strings,
    // where each String is a status update from the the thread.
    // Note that it is a String for this specific example because that is 
    // what was declared in the channel.
    for status_update in rx {
        println!("status_update: {status_update}");
    }

    // The Receiver<T> implements the Iterator trait.
    // The loop internally calls rx.recv() repeatedly.
    // .recv() is blocking by default, 
    // meaning it waits until a message is available.

    //_________________________________________________________________________
    
    // STEP: 4 => Add the JoinHandle to ensure that the main thread does not
    // exit before the loading_player_rankings_thread has finished as well.

    loading_player_rankings_thread.join().unwrap();

    //_________________________________________________________________________

    println!("The main thread is done");
}

// NOTE: This should be the output

// status_update: Loading player ranking data...
// status_update: 25% done
// status_update: 50% done
// status_update: 75% done
// status_update: The loading_player_rankings_thread is done.
// The main thread is done
