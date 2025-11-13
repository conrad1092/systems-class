// Shared academic info for all students
#[derive(Clone, Debug)]
struct AcademicProfile {
    major: String,
    gpa: f32,
}

// Trait: program to behavior only
trait ShowInfo {
    fn show_info(&self) -> String;
}

// Undergraduate student
#[derive(Clone, Debug)]
struct Undergraduate {
    name: String,
    profile: AcademicProfile,
    year: u8, // e.g., 1..=4
}

impl ShowInfo for Undergraduate {
    fn show_info(&self) -> String {
        format!(
            "Undergrad: {name}\n  Major: {major}\n  GPA: {gpa:.2}\n  Year: {year}",
            name = self.name,
            major = self.profile.major,
            gpa = self.profile.gpa,
            year = self.year
        )
    }
}

// Thesis component (grad-specific)
#[derive(Clone, Debug)]
struct Thesis {
    title: String,
    advisor: String,
}

// Graduate student
#[derive(Clone, Debug)]
struct Graduate {
    name: String,
    profile: AcademicProfile,
    thesis: Thesis,
}

impl ShowInfo for Graduate {
    fn show_info(&self) -> String {
        format!(
            "Graduate: {name}\n  Major: {major}\n  GPA: {gpa:.2}\n  Thesis: \"{title}\"\n  Advisor: {advisor}",
            name = self.name,
            major = self.profile.major,
            gpa = self.profile.gpa,
            title = self.thesis.title,
            advisor = self.thesis.advisor
        )
    }
}

// Enrollment that stores both undergrads and grads together
struct Enrollment {
    roster: Vec<Box<dyn ShowInfo>>,
}

impl Enrollment {
    fn new() -> Self {
        Self { roster: Vec::new() }
    }

    // Enroll any type that implements ShowInfo (no conditionals)
    fn enroll<T: ShowInfo + 'static>(&mut self, student: T) {
        self.roster.push(Box::new(student));
    }

    // Apply behavior uniformly to all enrolled students
    fn show_all(&self) -> String {
        self.roster
            .iter()
            .map(|s| s.show_info())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

// --- Example usage ---
fn main() {
    let cs_ug = Undergraduate {
        name: "Alex Rivera".into(),
        profile: AcademicProfile {
            major: "Computer Science".into(),
            gpa: 3.67,
        },
        year: 3,
    };

    let ee_grad = Graduate {
        name: "Sam Patel".into(),
        profile: AcademicProfile {
            major: "Electrical Engineering".into(),
            gpa: 3.91,
        },
        thesis: Thesis {
            title: "Low-Power DSP Architectures".into(),
            advisor: "Dr. Nguyen".into(),
        },
    };

    let mut enrollment = Enrollment::new();
    enrollment.enroll(cs_ug);
    enrollment.enroll(ee_grad);

    println!("{}", enrollment.show_all());
}
