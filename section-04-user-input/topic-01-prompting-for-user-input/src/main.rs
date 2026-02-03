/*
    ABOUT: Prompting for user input

*/

// Brings the `stdin()` function from the `io` module of the Rust standard
// library into scope.
use std::io::stdin;

fn main() {

    // This is a an empty String that will be used as a placeholer.
    // It will be replaced by the user's input.
    let mut raw_user_input: String = String::new();

    // Prompt the the user for an input
    println!("Enter your favourite number:");

    // The `stdin()` function will create an `Stdin` struct that has methods
    // for reading user input from the command line.
    stdin()
        // This will read the user input and store it in the variable
        // `raw_user_input`
        .read_line(&mut raw_user_input)
        .expect("Failed to read user input");

    let trimmed_user_input: &str = raw_user_input.trim();

    let favourite_number: u32 = trimmed_user_input.parse().expect(
        "trimmed_user_input could not be converted into an u32 data type",
    );

    println!("favourite_number: {favourite_number}");
}
