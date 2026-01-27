/*
    ABOUT: Code Blocks

*/

fn main() {

    let full_name: String = {
        // So in this context, this code block works like 
        // an inline function (aka an annonymous function).

        let first_name: &str = "Dezly";
        let last_name: &str = "Macauley";

        // The format macro is like println!() but it returns a String.
        format!("{} {}", first_name, last_name)
    };

    println!("My full name is {}", full_name);

}
