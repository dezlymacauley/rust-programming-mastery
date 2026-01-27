/*
    ABOUT: Functions that return multiple values

*/

fn main() {
    let value_a: i32 = 2;
    let value_b: i32 = 5;

    let (doubled_a, doubled_b) = double_each_number(value_a, value_b);
    println!("{value_a} doubled is {doubled_a}");
    println!("{value_b} doubled is {doubled_b}");
    // 2 doubled is 4
    // 5 doubled is 10

    // How to use a function that returns multiple values, 
    // when you only care about one of the values

    // E.g. I only care about the first value returned by the function
    let (player_one_name, _): (String, u32) = fighter_info();
    println!("player_one_name: {player_one_name}");
    // player_one_name: Goku

    // E.g. I only care about the second value returned by the function
    let (_, player_one_strength): (String, u32) = fighter_info();
    println!("player_one_strength: {player_one_strength}");
    // player_one_strength: 18531

}

//_____________________________________________________________________________

// SECTION: I will place all the custom functions here

// This is a function that has multiple inputs and outputs
// When you want a function to return multiple data types,
// then you have to use a tuple `()` and specify the data type of each
// element that is returned by the function
fn double_each_number(num1: i32, num2: i32) -> (i32, i32) {
    let doubled_num1 = num1 * 2;
    let doubled_num2 = num2 * 2;

    (doubled_num1, doubled_num2)
}

// This is a function that has multiple outputs
// To keep the example simple, 
// I'm only to hardcode the values that it returns.
fn fighter_info() -> (String, u32) {
    let fighter_name: String = String::from("Goku");
    let power_level: u32 = 18531; 

    (fighter_name, power_level)
}

//_____________________________________________________________________________
