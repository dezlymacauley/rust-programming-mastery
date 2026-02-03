/*
    ABOUT: src/lib.rs

    src/lib.rs is a module

    A module can be one of two things:
    - A single file like `src/lib.rs` that contains Rust code that can
    be imported into `src/main.rs`
    - A directory that contains multiple files that contain Rust code that
    can be import into `src/main.rs`

    There are three ways of adding code to `src/lib.rs`:
    - 1. Direct code declaration: 
    Put the code directly in the `src/lib.rs` module

    - 2. Single file module declaration: 
    Declare a file that contains some Rust code as a module.

    - 3. Multi-file module declaration:

*/

//_____________________________________________________________________________

// SECTION: Example 1 - Direct code declaration:

// The `pub` keyword makes an item public.
// Public means that the item can be accessed outside of the module that it
// was declared in.

pub fn say_hello(){
    println!("Hello from src/lib.rs");
}

//_____________________________________________________________________________

// SECTION: Example 2 - Single file module declaration:

// Create the file: `src/my_single_file_module.rs`

// This is how you declare the file `src/my_single_file_module.rs`as a module
pub mod my_single_file_module;

//_____________________________________________________________________________

// SECTION: Example 3 - Multi-file module declaration:

//_____________________________________________________________________________

// STEP: 1 - Create the file to represent the root of your
// multi-file module: 

// `src/my_multi_file_module.rs`
// Think of this file as the contents page where all of the files that the
// module contains will be listed.

//_____________________________________________________________________________

// STEP: 2 - Add this `my_multi_file_module.rs` file to lib.rs 
// to register it as a module in your program.

pub mod my_multi_file_module;

//_____________________________________________________________________________

// STEP: 3 - Create a directory for the multiple files 
// in the multi-file module: `src/my_multi_file_module`

// WARNING: Please make sure that the words in the directory name 
// are separated by underscores and NOT hyphens.

// When it comes package names:
// E.g. `cargo new name-of-project`
// Rust has no issues with that. That's why you can create a repo name on
// GitHub that contains hyphens.

// However when it comes to module names, 
// Rust only allows words in the module name to be separated by underscores.

//_____________________________________________________________________________

// STEP: 4 - Add some files to the `my_multi_file_module` directory

/*
    src/
    ├── my_multi_file_module/
    │   ├── file_01.rs
    │   └── file_02.rs
    └── my_multi_file_module.rs
*/

//_____________________________________________________________________________

// STEP: 5 - Open the `my_multi_file_module.rs` file 
// and declare all of the files inside
// the the `my-multi-file-module` directory as modules.

/*
    src/my_multi_file_module.rs
    ```
    pub mod file_01;
    pub mod file_02;
    ```
*/

//_____________________________________________________________________________
