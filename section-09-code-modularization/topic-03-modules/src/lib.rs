/*
    ABOUT: src/lib.rs

    src/lib.rs is a module

    A module can be one of two things:
    - A single file like `src/lib.rs` that contains Rust code that can
    be imported into `src/main.rs`
    - A directory that contains multiple files that contain Rust code that
    can be import into `src/main.rs`

    There are three ways of adding code to `src/lib.rs`:
    - Method 1: Put the code directly in the `src/lib.rs` module
    - Method 2: Declare a file that contains some Rust code as a module.
    - Method 3: multi-file example
*/

//_____________________________________________________________________________

// SECTION: Method 1 - Example:
// Put the code directly in `src/lib.rs`.

// The `pub` keyword makes an item public.
// Public means that the item can be accessed outside of the module that it
// was declared in.

pub fn say_hello(){
    println!("Hello");
}

//_____________________________________________________________________________

// SECTION: Method 2 - Example: 
// Declare a file that contains some Rust code as a module.

// This is how you declare the file `src/module_01.rs`as a module
pub mod module_01;

//_____________________________________________________________________________



//_____________________________________________________________________________
