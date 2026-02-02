/*
    ABOUT: Multiple Modules

*/

// This will use the `say_hello` function from:
// src/module_01.rs
use topic_02_multiple_modules::module_01::say_hello;

// This will use the `say_goodbye` function from:
// src/module_02.rs
use topic_02_multiple_modules::module_02::say_goodbye;

fn main() {
    say_hello();
    say_goodbye();
}
