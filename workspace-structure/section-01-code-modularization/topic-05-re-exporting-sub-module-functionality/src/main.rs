/*
    ABOUT: Re-exporting sub module functionality

    //_________________________________________________________________________

    E.g
    This is how to import functions from the sub-modules,
    src/my_multi_file_module/file_01.rs 
    and src/my_multi_file_module/file_02.rs

    my_multi_file_module::{
        file_01::hello_from_file_01,
        file_02::hello_from_file_02,
    }

    If you wanted the functions to be directly available from 
    my_multi_file_module, you just need to do this.

    Open the `src/my_multi_file_module.rs`:
    ```
        pub mod file_01;
        pub mod file_02;
    ```

    Add these lines to the end of the file:
    pub use file_01::hello_from_file_01;
    pub use file_02::hello_from_file_02;

    //_________________________________________________________________________

    Now you can do this:

    my_multi_file_module::{
        // This is how to import functions from the sub-modules,
        // src/my_multi_file_module/file_01.rs 
        // and src/my_multi_file_module/file_02.rs
        hello_from_file_01, hello_from_file_02
    }

    //_________________________________________________________________________

*/

use lib_dot_rs::{
    // This will import the `say_hello()` function from `src/lib.rs`
    say_hello,
    
    my_single_file_module::{
        // This is how to import the functions `say_good_morning()`,
        // and `say_good_night()` from the file src/my_single_file_module.rs
        say_good_morning, say_good_night
    },

    my_multi_file_module::{
        // This is how to import functions from the sub-modules,
        // src/my_multi_file_module/file_01.rs 
        // and src/my_multi_file_module/file_02.rs
        hello_from_file_01, hello_from_file_02
    }
};

fn main() {
    say_hello();
    say_good_morning();
    say_good_night();
    hello_from_file_01();
    hello_from_file_02();
}
