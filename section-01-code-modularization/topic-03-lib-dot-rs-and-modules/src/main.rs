/*
    ABOUT: Modules

*/

// This is how you import code from src/lib.rs
// I am importing the `say_hello()` function from `src/lib.rs`
use topic_03_lib_dot_rs_and_modules::say_hello;

// This is how to import the function `say_good_morning()` from the file
// src/my_single_file_module.rs that was declared as a module in src/lib.rs
use topic_03_lib_dot_rs_and_modules::my_single_file_module::say_good_morning;

// This is how to import the function `say_good_night()` from the file
// src/my_single_file_module.rs that was declared as a module in src/lib.rs
use topic_03_lib_dot_rs_and_modules::my_single_file_module::say_good_night;

// This is how to import the function `hello_from_file_01()` from the file
// src/my_multi_file_module/file_01.rs that was declared as 
// a module in src/lib.rs
use topic_03_lib_dot_rs_and_modules::my_multi_file_module::file_01::hello_from_file_01;

// This is how to import the function `hello_from_file_02()` from the file
// src/my_multi_file_module/file_02.rs that was declared as 
// a module in src/lib.rs
use topic_03_lib_dot_rs_and_modules::my_multi_file_module::file_02::hello_from_file_02;

fn main() {
    say_hello();
    say_good_morning();
    say_good_night();
    hello_from_file_01();
    hello_from_file_02();
}
