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

    // SECTION: How to remove an element at a specific index

    let index_to_remove: usize = 4;

    if index_to_remove < list_of_names.len() {
        list_of_names.remove(index_to_remove);
    } else {
        println!("There is no element at index {index_to_remove}");
        println!(
            "The last index in the list is at index {}",
            (list_of_names.len() - 1)
        );
    }

    println!("list_of_names: {list_of_names:?}");

    //_________________________________________________________________________

    // SECTION: How to check if an element exists inside a vector

    let deployment_options: Vec<String> = vec![
        String::from("aws"),
        String::from("deno-deploy"),
        String::from("gcp"),
    ];

    let search_term: String = String::from("aws");

    // .contains() accepts a reference to the data type of the elements in
    // the vector, and it returns a bool value (true or false).
    //
    // deployment_options is `Vec<String>` so to use `.contains()` to 
    // check if a value exists in it, 
    // then you would have to give the `.contains()` the data type `&String`
    println!(
        "Is {search_term} a deployment option? {}",
        deployment_options.contains(&search_term)
    );
    // Is aws a deployment option? true

    //_________________________________________________________________________
}
