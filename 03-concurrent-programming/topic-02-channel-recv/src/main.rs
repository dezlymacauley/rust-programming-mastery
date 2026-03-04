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

    // TIP: Don't forget the `move` keyword. The closure should take control
    // of the the sender variable, to ensure that the sender variable lives
    // for as long as the thread is valid.

    let task_a_thread = spawn(move || {
        // Imagine it takes 1 second to start Task A
        sleep(Duration::from_secs(1));

        // The task A sends the first status update message,
        // to rx (The receiver)
        tx.send("Started Task A...".to_string()).unwrap();

        // Imagine it takes 2 seconds to complete Task A
        sleep(Duration::from_secs(2));

        // The task A then sends the second status update message,
        // to rx (The receiver)
        tx.send("Task A completed.".to_string()).unwrap();
    });

    //_________________________________________________________________________

    // STEP: 5 => Use `rx` to receive each message

    // NOTE: The `.recv()` method is a blocking method.
    // Each time you calle `.recv()`, 
    // it will pause the thread `fn main()` until it receives the message
    // sent by the `task_a_thread`.

    let first_status_message = rx.recv().unwrap();
    println!("{first_status_message}");

    let second_status_messsage = rx.recv().unwrap();
    println!("{second_status_messsage}");

    // NOTE: This is not the best way to receive messages,
    // I'm simply doing it this way to keep it simple.

    //_________________________________________________________________________

    // STEP: 4 => Add the JoinHandle to ensure that the main thread
    // does not exit before the program ends

    println!("Task B completed.");
    println!("Task C completed.");
    println!("Task D completed.");
    
    task_a_thread.join().unwrap();

    //_________________________________________________________________________
}

/*

    NOTE: This should be the final result

    Started Task A...
    Task A completed.
    Task B completed.
    Task C completed.
    Task D completed.

*/
