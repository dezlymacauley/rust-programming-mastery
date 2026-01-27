#[derive(Debug)]
pub struct Student {
    id: u8,
    pub name: String,
    pub age: u8,
}

// This is an implementation block
// It is used to create `methods` (aka associated functions)
impl Student {
   
    // Constructor method.
    // It simplifies the syntax of creating an instance of a struct.
    // `Self` is the same as `Student`
    pub fn new(student_name: &str, student_age: u8) -> Self {

        Self {
            id: 0,
            name: String::from(student_name),
            age: student_age
        }

    }

    // Getter method (`&self`)
    // Use this when you only want to read a value 
    // from an instance of a struct
    pub fn view_id(&self) -> u8 {
        self.id 
    }

    // Setter method (`&mut self`)
    // Use this when you want to modify the value of a struct
    pub fn change_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

}
