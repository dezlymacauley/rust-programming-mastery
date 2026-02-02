/*
    The simplest code modularization structure in Rust is to simply put all
    your resuable code directly in lib.rs
*/

// Make sure to put the `pub` keyword before the name of a function if you
// intend to use this function outside of this file.
pub fn say_hello() {
    println!("Hello")
}
