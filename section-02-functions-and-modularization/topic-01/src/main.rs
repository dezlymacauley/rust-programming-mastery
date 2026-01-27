/*
    ABOUT: Functions

    A function is a named block of code that optionally accepts input,
    performs some actions, and optionally returns a value a multiple values.

    - Some functions don't accept any inputs.
    - Some functions don't return any output back
    to the caller of the function.
    - Some functions return multiple values (I will have a separate lesson
    on this).

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
    say_hello();
    // Hello
    //
    greet_user("Dezly", "Macauley");
    // Hello Dezly Macauley

    let num_apples: i32 = 20;
    let num_peaches: i32 = 10;

    let total_fruits: i32 = add_two_integers(num_apples, num_peaches);
    println!("{num_apples} + {num_peaches} = {total_fruits}");
    // 20 + 10 = 30

    let value_a: i32 = 2; 
    let value_b: i32 = 5; 

    let (doubled_a, doubled_b) = double_each_number(value_a, value_b);
    println!("{value_a} doubled is {doubled_a}");
    println!("{value_b} doubled is {doubled_b}");
    // 2 doubled is 4
    // 5 doubled is 10
}

//_____________________________________________________________________________

// SECTION: I will place all the custom functions here

// Note that Rust does not care whether you have the function declaration,
// below or above `fn main()`

// This function accepts no inputs, and returns no data back to the caller.
fn say_hello() {
    println!("Hello");
}

// This function accepts multiple inputs,
// and returns no data back to the caller.
fn greet_user(first_name: &str, last_name: &str) {
    println!("Hello {first_name} {last_name}");
}

// This function accepts multiple inputs,
// and returns data back to the caller.
// You would use this function to assign value to a variable.
fn add_two_integers(num1: i32, num2: i32) -> i32 {
    // In Rust the last expression in a funtion is returned to the caller.
    // An expression does not end with a semicolon because that would make
    // it a statement.
    num1 + num2

    // In other programming language using a `return` statement is used.
    // This is also valid in Rust.
    // return num1 + num2;
}

// This is a function that has multiple inputs and outputs
// When you want a function to return multiple data types,
// then you have to use a tuple `()` and specify the data type of each
// element that is returned by the function
fn double_each_number(num1: i32, num2: i32) -> (i32, i32) {
    let doubled_num1 = num1 * 2;
    let doubled_num2 = num2 * 2;

    (doubled_num1, doubled_num2)
}

//_____________________________________________________________________________
