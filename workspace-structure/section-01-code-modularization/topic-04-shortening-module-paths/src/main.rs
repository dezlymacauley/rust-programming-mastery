/*
    ABOUT: Shortening Module Paths

    //_________________________________________________________________________

    // TIP: 1 - Create an alias for lib.rs,
    // instead of using the ful package name.

    Add these lines to the Cargo.toml file at the root of your project

    [lib]
    name = "lib_dot_rs"

    Then this:
    use topic_04_shortening_module_paths::say_hello;

    Can be shortened to this:
    use lib_dot_rs::say_hello;

    Not only is this a cleaner import, but you don't have to worry about
    imports breaking if the name of your package changes.

    //_________________________________________________________________________

    // TIP: 2 - Use grouped imports

*/

use lib_dot_rs::{
    // This will import the `say_hello()` function from `src/lib.rs`
    say_hello,
    
    my_single_file_module::{
        // This is how to import the functions `say_good_morning()`,
        // and `say_good_night()` from the file src/my_single_file_module.rs
        say_good_morning,
        say_good_night
    },

    my_multi_file_module::{
        // This is how to import functions from the sub-modules,
        // src/my_multi_file_module/file_01.rs 
        // and src/my_multi_file_module/file_02.rs
        file_01::hello_from_file_01,
        file_02::hello_from_file_02,
    }
};

fn main() {
    say_hello();
    say_good_morning();
    say_good_night();
    hello_from_file_01();
    hello_from_file_02();
}
