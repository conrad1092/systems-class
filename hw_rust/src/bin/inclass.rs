struct Student {
    name: String,
    major: String,
}

impl Student {
    // "Constructor"-like method (Rust uses associated functions, often called new)
    fn new(name: &str, major: &str) -> Student {
        Student {
            name: name.to_string(),
            major: major.to_string(),
        }
    }

    // Method to set the major
    fn set_major(&mut self, major: &str) {
        self.major = major.to_string();
    }

    // Method to get the major
    fn get_major(&self) -> &str {
        &self.major
    }
}

fn main() {
    // Create a Student using the "constructor"
    let mut s1 = Student::new("Alice", "Computer Science");

    // Print initial major
    println!("{}'s major: {}", s1.name, s1.get_major());

    // Change major
    s1.set_major("Electrical Engineering");

    // Print updated major
    println!("{}'s new major: {}", s1.name, s1.get_major());
}
