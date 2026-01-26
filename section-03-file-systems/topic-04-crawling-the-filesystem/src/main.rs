use std::env;
use std::fs::read_dir;
use std::path::Path;

// Accepts a refences to a Path. 
// Path::new() returns this data type
fn walk_path(path: &Path) {

    // Assume the path is a directory for this example
    for entry in read_dir(path).unwrap() {

        let entry = entry.unwrap();

        println!("Entry Path: {}", entry.path().display());

        let path = entry.path();

        if path.is_dir() {
            // Using recursion to go a level deeper by calling
            // this function again.
            walk_path(&path);
        } else {
            // If the path is not a directory then print it out.
            println!("{}", path.display());
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let path: &Path = Path::new(&args[1]);

    if !path.exists() {
        println!("Path {} does not exist", path.display());
    }

    walk_path(path);

}
