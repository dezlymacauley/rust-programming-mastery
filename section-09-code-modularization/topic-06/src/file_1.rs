fn some_fn() {
    println!("hey");
}


// module
mod maths {
    
    // sub module
    pub mod basic_maths {

        pub fn multiplication(num1: &i32, num2: &i32) -> i32 {
            let result: i32 = num1 * num2;
            printing(&result);
            result
        }

        fn printing(num: &i32) {
            println!("The result is {}", num);
            crate::file_1::some_fn();
        }

    }

}

pub fn rect_area(length: &i32, width: &i32) -> i32 {
    use maths::basic_maths::multiplication;
    multiplication(length, width)
}
