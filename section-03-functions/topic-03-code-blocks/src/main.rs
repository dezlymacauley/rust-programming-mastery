/*
    ABOUT: Code Blocks

*/

fn main() {

    let full_name: String = {
        let first_name: &str = "Dezly";
        let last_name: &str = "Macauley";

        // The format macro is like println!() but it returns a String.
        // Like a function, 
        // the last expression in a code block is automatically returned.
        format!("{} {}", first_name, last_name)
    };

    println!("My full name is {}", full_name);

}
