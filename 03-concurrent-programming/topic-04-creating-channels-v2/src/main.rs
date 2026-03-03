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
use std::sync::mpsc::channel;

use std::thread::spawn;

fn main() {

    // Each channel consists of two parts:
    // 1. A `sender` variable, which is usually named tx (transmitted)
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
    let (tx, rx) = channel();

    // This creates a new thread
    spawn(move || {
        // Because of the `move` keyword the variable `tx` (the sender) 
        // is transfered into the thread.
        tx.send("Hello from spawned tread").unwrap();
    });

    // There is no need for a join handle because the  `.recv()` method
    // blocks the main thread, until a message is recieved 
    // from the spawned thread.

    //_________________________________________________________________________

    // SECTION: Method 1

    // let recieved = rx.recv().unwrap();
    // println!("Got: {recieved}");
    
    //_________________________________________________________________________
    
    // SECTION: Method 2

    match rx.recv() {
        Ok(message) => println!("recieved: {message}"),
        Err(error) => println!("An error occured: {error}")
    }
    
    //_________________________________________________________________________

}
