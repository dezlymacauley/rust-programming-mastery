use topic_02_impl_blocks::Student;

fn main() {

    let new_student = Student {
        name: "zack".to_string(),
        age: 21,
    };

    println!("new_student: {new_student:?}");
    // new_student: Student { name: "zack", age: 21 }

    println!();

    println!("new_student: {new_student:#?}");
    /*
        new_student: Student {
            name: "zack",
            age: 21,
        }
    */
}
