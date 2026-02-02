mod person {

    pub struct PersonalInfo {
        pub age: u8,
        pub education: String
    }

    impl PersonalInfo {
        pub fn new(new_edu: &str) -> Self {
            Self {
                education: String::from(new_edu),
                age: 20
            }
        }
    }

}

pub fn some_person() {
    let mut person1 = person::PersonalInfo::new("Bachelors");
    person1.education = String::from("Masters");

    let person2: PersonalInfo = person::PersonalInfo {
        age: 42,
        education: String::from("Masters")
    };
}
