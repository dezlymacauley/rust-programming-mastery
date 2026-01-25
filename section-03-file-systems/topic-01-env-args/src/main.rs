/*
    ABOUT: The `env::args()` function

    //_________________________________________________________________________

    The `env::args()` functions returns the data type `Args`,
    which is an iterator.

    An iterator in simple terms, is a list of elements that has been
    processed to make it convenient to target each element in the list.

    let iterator_of_cli_arguments: Args = env::args();

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
