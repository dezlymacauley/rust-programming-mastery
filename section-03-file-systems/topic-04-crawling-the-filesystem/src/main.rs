use std::env;
use std::path::Path;

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

}
