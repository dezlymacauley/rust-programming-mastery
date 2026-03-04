/*
    ABOUT: Process and thread

    What is a process?

    A process is an isolated area in memory for a program.
    It includes things like:

    Memory allocated for the program (variables, data, stack, heap)
    Resources like file handles, network connections
    Execution state (program counter, registers, etc.)

    Think of a process as a self-contained “container” where a program runs.
    Each process is isolated from others,
    so one crashing doesn’t usually crash the others.

    The code of a running program operates within a process,
    and these processes can run concurrently.

    What is a thread?
    A process can have multiple independent parts
    that are executing code simultaneously. These are called threads.

    A thread is a single path of execution inside a process.
    Threads share the process’s memory and resources
    but execute independently.

*/

use std::thread::{sleep, spawn, JoinHandle};

use std::time::Duration;

fn main() {
    let task_a_thread: JoinHandle<()> = spawn(|| {
        // Imagine it takes 1 second to start Task A
        sleep(Duration::from_secs(1));
        println!("Started Task A...");

        // Imagine it takes 2 seconds to complete Task A
        sleep(Duration::from_secs(2));
        println!("Task A completed.");
    });

    println!("Task B completed.");
    println!("Task C completed.");
    println!("Task D completed.");
    
    // `.join()` is a blocking method.
    // So the thread `fn main()` will be paused until the thread
    // `task_a_thread` completes all of its tasks.
    task_a_thread.join().unwrap();
}

/*
    NOTE: This should be the output

    Task B completed.
    Task C completed.
    Task D completed.
    Started Task A...
    Task A completed.

*/
