/*
    ABOUT: Creating a vector with repeated values

*/

fn main() {
    
    // This will create a Vector of three u16 data types,
    // and each of the three elements will have the value of 100.
    let list_of_numbers: Vec<u16> = vec![100;3];

    println!("list_of_numbers: {list_of_numbers:?}");
    // list_of_numbers: [100, 100, 100]
}
