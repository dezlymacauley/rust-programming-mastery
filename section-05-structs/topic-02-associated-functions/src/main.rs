use topic_02_associated_functions::Student;

fn main() {
    let mut new_student = Student::new("zack", 21);

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

    // How to use the `getter` method
    println!("new_student id is {}", new_student.view_id());
    // new_student id is 0

    // How to use the `setter` method
    new_student.change_name("yuta");    
    println!("new_student name is {}", new_student.name);
    // new_student name is yuta

}
