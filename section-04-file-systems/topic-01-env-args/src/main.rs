/*
    ABOUT: The `env::args()` function

    //_________________________________________________________________________

    The `env::args()` functions returns the data type `Args`,
    which is an iterator.

    An iterator in simple terms, is a list of elements that has been
    processed to make it convenient to target each element in the list.

    let iterator_of_cli_arguments: Args = env::args();

    //_________________________________________________________________________
    
    // ABOUT: What is the point of env::args() ?

    When you enter text into the command line (aka terminal), 
    the shell (this can be bash, zsh or something else) parses it.

    Two major rules are applied:
    - Whitespace separates arguments
    - Quotes are used to prevent an argument from splitting

    - Some non-Unix shells may behave differently.

    The raw arguments are collected as a list (aka array) 
    of text-like values provided by the OS.
    
    E.g. If you type dezly macauley, the list becomes:
    "program path", "dezly", "macauley"

    After the shell has done this, it then calls the OS 
    to start your program.

    Depending on the operating systems, the list of text
    containing the cli arguments may be handled and formatted differently.

    So for safety reasons and cross-platform compatibility,
    `env::args()` exposes those OS-provided arguments 
    as the data type `Args`

    `env::args()` prepares the CLI arguments for use in Rust,
    by converting each argument into an owned UTF-8 String, 
    that can be iterated over.

    If the cli arguments are not valid Unicode, then `env::args()` may fail.

    `Iterated over` simply means that you can go through the list and read
    the value of each element in the list and perform some action on each
    element.

    An iterator does not have indexes, it just provides a way to go to
    the next element until you reach the end:

    It would look something like this:
    next() -> Option<Item>
    next() -> Option<Item>
    next() -> Option<Item>
    next() -> None

    To store the arguments from `env::args()` into a variable,
    you need to use the `.collect()` method and specify what format you
    want the list to be in. E.g. `Vec<String>`

    Once it is in this Vector form then you can target a specific element by
    its index.

    E.g. If you the program was run with the argument dezly macauley, 
    then the Vector would be:

    index 0,        index 1   index 2
    "program path", "dezly", "macauley"
    
    //_________________________________________________________________________

    // ABOUT: What is the difference between env:args() and std::read_line()

        So .read_line() is used when you want to get user input from 
        the user After the program has already started. This input will,
        then control what happens next.

        env:args is used when you want to supply command line arguements 
        BEFORE the program is run. Those arguments arguments will change 
        how the program behaves when it starts.

    //_________________________________________________________________________

*/

// Bring the `env` module from the Rust standard library into scope
use std::env;

fn main() {
    // `.collect()` creates a list from the iterator.
    // You have to specify the type of list when using `.collect()`
    let cli_arguments: Vec<String> = env::args().collect();

    // When it comes to cli arguments,
    // The first argument which is at index 0 will always contains 
    // the path to the binary of the program.
    println!("First argument: {}", cli_arguments[0]);

    // The next argument will be at index 1
    // To enter this: `cargo rq some-text`
    println!("Second argument: {}", cli_arguments[1]);

    // The next argument will be at index 2
    // To enter this: `cargo rq some-text some-more-text`
    println!("Third argument: {}", cli_arguments[2]);

    //_________________________________________________________________________

    // Since `cli_arguments` is a vector,
    // you can use the `.len()` method to get the total number of elements
    // in the vector.

    // For cli_arguments, the number of arguments will always be at least 1,
    // because the first element is a String that contains the path to the
    // binary of the program.
    println!("Number of arguments: {}", cli_arguments.len());
    // Number of arguments: 3

    // NOTE: This code will not compile if you run the program without
    // supplying all the required arguments.

    // E.g. This will work
    //  index 0    index 1   index 2:
    // `cargo rq` `dezly`   `macauley`
    
    // E.g. This will not work because because index 1, and 2 are missing
    // `cargo rq`

    //_________________________________________________________________________

    // How to view all the arguments (single line format)
    println!("All arguments: {cli_arguments:?}");
    
    println!();
    
    // How to view all the arguments (multi-line format)
    println!("All arguments: {cli_arguments:#?}");
}
