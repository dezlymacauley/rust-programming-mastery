/*
    ABOUT: The Rust module system

    package:
        - crates:

    package
    - A package is a bundle of one or more crates 
    that provides a set of functionality. 
    - contains a Cargo.toml file

    A crate is the smallest amount of code that 
    the Rust compiler considers at a time. 

    There are two types of crates:
    1. binary crate
    2. library crate

    binary crate:
    - contain a `main` function.
    - programs that compile to an executable binary that you can run.
    
    library crate:
    - does not have a `main` function
    - meant to be used by importing it into a binary crate

    ___________________________________________________________________________
    
    When you run the command `cargo new name-of-crate`,
    a binary crate will be created by default
    ___________________________________________________________________________
    .
    ├── Cargo.toml
    └── src
        └── main.rs
    ___________________________________________________________________________

    This is Cargo.toml:

    [package]
    name = "topic-01-the-rust-module-system"
    version = "0.1.0"
    edition = "2024"

    [dependencies]
    ___________________________________________________________________________
   
    Here's the breakdown:

    package = The package is `new topic-01-the-rust-module-system`
    
    crate(s) = There is only one crate in this example. `src/main.rs`
    And this crate is a binary crate.

    main.rs tells Rust where to find the function `main()`,
    which tells Rust where to start executing the program. 

    ___________________________________________________________________________
*/

fn main() {
    println!("Hello, world!");
}
