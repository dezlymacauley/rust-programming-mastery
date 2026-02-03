/*
    ABOUT: Creating a reference of a section of a vector 

*/

fn main() {
    
    let list_of_numbers: Vec<u16> = vec![10, 20, 30, 40, 50, 60];

    let first_three_numbers: &[u16] = &list_of_numbers[0..=2];

    // This will create an immutable reference (read-only)
    // of the elments starting from index 0 (including index 0),
    // to index 2 (including index 2).
    
    // WARNING: Don't forget the `=` sign.
    // This is what makes index 2 (inclusive)

    // 0..=2 means give me the elements at:
    // index 0, (first number)
    // index 1, (second number)
    // and index 2 (third number)

    println!("first_three_numbers: {first_three_numbers:?}");
    // first_three_numbers: [10, 20, 30]

}
