/*
    ABOUT: Topic 01 - lib.rs

    To import `src/lib.rs` into `src/main.rs`,
    the syntax is:

    use name_of_your_project;

    Note: Rust will automatically convert any hyphens in your project
    name to an underscore.

*/

// The statment `use name_of_your_project` will always start from
// the path `src/lib.rs`.
// Any code inside `src/lib.rs` that has been marked as public with the
// `pub` keyword can be used like this. 
use topic_01_lib_dot_rs::say_hello;

fn main() {
    say_hello();
}
