/*
    ABOUT: Closure - Ownership

    Closures can take ownership of the variables that they capture,
    by using the keyword `move` within the closure.
    
    The syntax is:
    let name_of_closure = move | parameters | -> return types { logic };

    When `move` is used:
    - The closure takes ownership of a capture variable
    - This transfer of ownership happens as soon as the variable is used
    within the declaration of the closure.
    - This means that unlike a regular function, 
    the closure does not need to called before 
    the ownership transfer happens.
    - Once a variable has been captured in a closure, 
    the variable will live inside the closure, 
    for as long as that closure is in scope.
*/

fn main() {
    // These variables will be captured (used) by the closures below,
    // with full ownership transferred when using the `move` keyword

    // I'll be using player_name, which is a non-Copy type,
    // and ninja_stars, which is a Copy data type because the `move`
    // keyword behaves differently for Copy data types, 
    // and non-Copy data types.

    let player_name: String = String::from("Nexus Legend");

    let ninja_stars: i32 = 8;

    //_________________________________________________________________________

    // SECTION: Example 1 - No parameters, no return types
    // Move of a non-Copy type

    // A non-Copy type is a data type like `String` where the contents of
    // that variable are stored on the Heap.

    // The variable `player_name` is valid before the closure is declared.
    println!();
    println!("player_name: {player_name}");
    // player_name: Nexus Legend

    let print_player_name = move || {
        // `player_name` is now OWNED by this closure
        println!("player_name: {player_name}");
    };

    // If you uncomment this line below the code will not compile.
    // The player_name variable can no longer be used.
    // println!("{player_name}"); 

    // This will work because `player_name` lives inside the closure.
    print_player_name();
    // player_name: Nexus Legend
  
    // As long as the the closure lives, the `player_name` within the closure
    // is valid.
    print_player_name();
    // player_name: Nexus Legend

    //_________________________________________________________________________
    
    // SECTION: Example 2 - Capturing Copy Types

    // For `Copy` types, Rust COPIES the value into the closure.
    // The original variable is still accessible after the closure is defined

    // A Copy type is a data type like an integer, unsigned integer,
    // float, bool, char or any data types where the value is stored entirely
    // on the STACK.

    println!();
    println!("ninja_stars: {ninja_stars}");
    // ninja_stars: 8
   
    let print_ninja_stars = move || {
        // `ninja_stars` is COPIED into this closure (i32 is a Copy type)
        println!("ninja_stars: {ninja_stars}");
    };

    // This will compile because the variable `ninja_stars` is a Copy type,
    // so even though the closure uses the `move` keyword,
    // `ninja_stars` is actually copied (and not moved) when the closure
    // copies it.
    println!("ninja_stars: {ninja_stars}");

    print_ninja_stars();
    
    println!("ninja_stars: {ninja_stars}");
    
    print_ninja_stars();

    //_________________________________________________________________________
}
