/*
    ABOUT: Vector methods

*/
fn main() {
   
    //_________________________________________________________________________
    
    // SECTION: How to view an element at a specific index
    
    let list_of_numbers: Vec<u16> = vec![15, 8, 27];
    // index 0 = 15
    // index 1 = 8
    // index 2 = 27

    let element_at_index_2: Option<&u16> = list_of_numbers.get(2);
    println!("element_at_index_2: {element_at_index_2:?}");
    // element_at_index_2: Some(27)

    let element_at_index_3: Option<&u16> = list_of_numbers.get(3);
    println!("element_at_index_3: {element_at_index_3:?}");
    // element_at_index_3: None

    //_________________________________________________________________________

    // SECTION: How to add elements to the end of a vector

    let mut list_of_names: Vec<String> = vec![
        String::from("Megumi"),
        String::from("Nobara"),
        String::from("Yuji"),
    ];

    list_of_names.push("Dezly".to_string());

    println!("list_of_names: {list_of_names:?}");
    // list_of_names: ["Megumi", "Nobara", "Yuji", "Dezly"]

    //_________________________________________________________________________

}
