/*
    ABOUT: Vectors

    A vector is an ordered collection of elements 
    that are of the same data type.
*/

fn main() {
    let list_of_names: Vec<String> = vec![
        String::from("Megumi"),
        String::from("Nobara"),
        String::from("Yuji"),
    ];
    
    // The vector data type implements the `debug` trait,
    // so you can print it out in a single line format (:?) 
    // or mult-line format (:#?)

    println!("list_of_names: {list_of_names:?}");
    // list_of_names: ["Megumi", "Nobara", "Yuji"]

    println!();

    println!("list_of_names: {list_of_names:#?}");
    /*

        list_of_names: [
            "Megumi",
            "Nobara",
            "Yuji",
        ]

    */

}
