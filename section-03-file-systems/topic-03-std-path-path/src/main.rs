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
    let file_path: &Path = Path::new("csv-files/example.csv");

    if file_path.exists() == false {
        println!("{} was not found.", file_path.display());

        // exit the program if the file path does not exist.
        return;
    }

    println!("file_path: {}", file_path.display());
    // file_path: csv-files/example.csv
}
