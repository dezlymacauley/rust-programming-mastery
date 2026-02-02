/*
    You can use `lib.rs` to list the modules in your project
    A module is simply a Rust file that contains reusable code.
    
    NOTE: The `pub` keyword only means that the files module_01.rs,
    and module_02.rs can be imported by code that is outside of this
    lib.rs file.
    -----------------------------------------------------------------
    The code inside each file must be marked with `pub` in order to
    use the functionality of the module
*/

// This is src/module_01.rs
pub mod module_01;

// This is src/module_02.rs
pub mod module_02;
