/*
    ABOUT: Accessing Vector elements

    You can access the elements of a vector by their index

    The first element is at index 0,
    the second element is at index 1, and so on.

    The last element is at index[number of elements - 1]

    E.g. If you have 3 names in a vector,
    the last element will be at index[3 - 1] = index 2

    You can use the built-in `.len()` method to get the number of elements
    in a Vector.

    WARNING: When trying to access elements in a Vector by their index,
    first perform a check to confirm that the index is available or your
    program will crash.
   
    I have deliberately using `happy path` programming, 
    over `defensive programming` to keep this example simple.
*/

fn main() {
    let mut list_of_names: Vec<String> = vec![
        String::from("Megumi"),
        String::from("Nobara"),
        String::from("Yuji"),
    ];

    println!("There are {} names in the list", list_of_names.len());
    // There are 3 names in the list

    println!("The first name is: {}", list_of_names[0]);
    // The first name is: Megumi

    println!("The second name is: {}", list_of_names[1]);
    // The second name is: Nobara

    println!(
        "The last name is: {}",
        list_of_names[list_of_names.len() - 1]
    );
    // The last name is: Yuji

    //_________________________________________________________________________

    // How to update an element in the Vector

    list_of_names[0] = String::from("Dezly");
    println!("The first name is: {}", list_of_names[0]);
    // The first name is: Dezly

    //_________________________________________________________________________
}
