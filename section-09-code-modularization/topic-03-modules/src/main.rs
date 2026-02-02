/*
    ABOUT: Modules

*/

// This is how you import code from src/lib.rs 
// I am importing the `say_hello()` function from `src/lib.rs`
use topic_03_modules::say_hello;


// This is how to import the function `say_goodbye()` from src/module_01.rs,
// that was declared as a module in src/lib.rs
use topic_03_modules::module_01::say_goodbye;

fn main() {
    say_hello();
    say_goodbye();
}  
