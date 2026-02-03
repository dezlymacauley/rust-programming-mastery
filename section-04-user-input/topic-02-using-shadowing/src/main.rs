/*
    ABOUT: Using shadowing

*/

// Brings the `stdin()` function from the `io` module of the Rust standard
// library into scope.
use std::io::stdin;

fn get_favourite_number() {

    // Shadowing is a tool in Rust that can be quite convinient 
    // when done safely. My general rule for shadowing, 
    // is to only use it in a function that has one specific goal.
  
    // Shadowing allows you to re-use a variable name by re-declaring it
    // with the let keyword.

    // The main use case for shadowing is when you want to get repeately,
    // transform a variable from one data type to another,
    // and you only care about using the final processed output.

    // Without shadowing I would have to create three variable names:
    // 1. raw_user_input
    // 2. trimmed_user_input
    // 3. trimmed_user_input_converted_to_u32

    // However I only care about the last output,
    // so shadowing is a good use case.

    let mut user_input: String = String::new();

    println!("Enter your favourite number:");

    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

    let user_input: &str = user_input.trim();

    let user_input: u32 = user_input
        .parse()
        .expect("user_input could not be converted into an u32 data type");

    println!("favourite_number: {user_input}");
}

fn main() {
    get_favourite_number();
}
