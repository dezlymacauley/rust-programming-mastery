/*
    ABOUT: Modules

*/

// This is how you import code from src/lib.rs 
// I am importing the `say_hello()` function from `src/lib.rs`
use topic_03_lib_dot_rs_and_modules::say_hello;

// This is how to import the function `say_good_morning()` from 
// src/single_file_module_01.rs that was declared as a module in src/lib.rs
use topic_03_lib_dot_rs_and_modules::single_file_module_01::say_good_morning;

// This is how to import the function `say_good_night()` from 
use topic_03_lib_dot_rs_and_modules::single_file_module_01::say_good_night;

fn main() {
    say_hello();
    say_good_morning();
    say_good_night();
}  
