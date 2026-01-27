/*
    ABOUT: The `std::path:Path` data type

    You can declare a file path as a string slice like this:
    let file_path: &str = "./csv-files/example.csv";
    println!("file_path: {file_path}");

    However Rust has a built-in data type for working with file path.

    `std::path::Path` is a struct that has methods for performing checks
    on a file path.

*/

// Brings that `Path` data type,
// from the `path` module of the Rust standard library into scope.
use std::path::Path;

fn main() {
    // Path::new() accepts a string slice &str or a reference to a String,
    // and returns &Path.
    //
    // Use Path::new() when you only need to read a path to work with it
    // and you don't care about ownership.
    //
    // If you do care about ownership and need to modify or join 
    // file paths, then use PathBuf instead.
    let file_path: &Path = Path::new("csv-files/example.csv");

    if file_path.exists() == false {
        println!("{} was not found.", file_path.display());

        // exit the program if the file path does not exist.
        return;
    }

    println!("file_path: {}", file_path.display());
    // file_path: csv-files/example.csv
}
