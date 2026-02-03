/*
    ABOUT: A package with multiple binary crates

    A package can have multiple binary crates.
    
    Each binary crates will function as its own standalone program,
    with its own main.rs

    To do this create the directory `src/bin/`

    Then place each crate inside the `bin` directory:

    .
    ├── Cargo.toml
    └── src
        └── bin
            ├── crate_01.rs
            └── crate_02.rs

    ___________________________________________________________________________

    To run the code in src/bin/crate_01.rs:
    cargo rq --bin crate_01                                 
    
    To run the code in src/bin/crate_02.rs:
    cargo rq --bin crate_02

*/

fn main() {
    println!("Hello from Crate 01");
}
