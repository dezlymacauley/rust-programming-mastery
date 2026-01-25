use topic_01_struct_basics::Student;

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
