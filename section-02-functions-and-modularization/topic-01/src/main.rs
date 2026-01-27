/*
    ABOUT: Functions
    
    A function is a named block of code that optionally accepts input, 
    performs some actions, and optionally returns a value a multiple values.

    - Some functions don't accept any inputs.
    - Some functions don't return any output back 
    to the caller of the function.
    - Some functions return multiple values.

    First you declare a function:
    - This is called the function definition. 
    - This is where you specify how the function should work.
    - When declaring a function, 
    the inputs that it accepts are called `parameters`

    Then you use the function in your code by calling the function:
    - This is called a function call
    - When using the function, 
    the input values that you supply to the function are called arguments.

*/


/*
    `main()` a special function that tells Rust where to start the execution
    of a program.

    In Rust functions that don't return any elements back to the caller will
    implicitly return a data type called a `unit`.
    
    A `unit` is simply an empty tuple. `()`

    So this:

    fn main() {

    }

    Is the same as this:

    fn main() -> () {

    }

    The arrow indicates the return type of the function.
*/

fn main() {
    println!("zzzHello, world!");
}

//_____________________________________________________________________________

// SECTION: I will place all the custom functions here 

// Note that Rust does not care whether you have the function declaration,
// below or above `fn main()`



//_____________________________________________________________________________
