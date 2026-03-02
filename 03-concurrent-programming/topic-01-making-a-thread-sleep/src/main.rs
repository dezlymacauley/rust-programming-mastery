/*
    ABOUT: Making a thread sleep

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

use std::thread::sleep;
use std::time::Duration;

fn main() {

    // This will iterate over the range of numbers from 1 to 3 (including 3)
    // In Rust the equal sign before 3, as you can see here `(1..=3)`,
    // makes 3 inclusive.
    // `.rev()` means that the `for loop` will go through the range 
    // in reverse order. So 3, 2, 1
    for i in (1..=3).rev() {
        // Print out each element starting from 3
        println!("{i}");

        // This will put the thread `main` to sleep for 1 second,
        // after printing out each number.
        sleep(Duration::from_secs(1));
    }
}
