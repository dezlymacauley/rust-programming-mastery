/*
    ABOUT: How to enforce the number of arguments

*/

use std::env;

fn main() {
    let cli_arguments: Vec<String> = env::args().collect();

    // I want exactly 3 arguments so I use the `!=` sign
    if cli_arguments.len() != 3 {
        println!("This program requires 3 arguments");

        // This is an early return to end the `main()` function immediately.
        return;
    }

    println!("First argument: {}", cli_arguments[0]);
    println!("Second argument: {}", cli_arguments[1]);
    println!("Third argument: {}", cli_arguments[2]);

    println!("Number of arguments: {}", cli_arguments.len());
}
