// Definition of the Student struct
struct Student {
    id: u32,
    name: String,
    email: String,
}

// Definition of the repository for data persistence
struct StudentRepository {
    students: Vec<Student>,
}

impl StudentRepository {
    // Method to save a new student
    fn save(&mut self, student: Student) {
        self.students.push(student);
    }

    // Method to get a student by ID
    fn get_by_id(&self, id: u32) -> Option<&Student> {
        for student in &self.students {
            if student.id == id {
                return Some(student);
            }
        }
        None
    }
}

fn main() {
    let mut repository = StudentRepository {
        students: Vec::new(),
    };

    // Saving some students
    repository.save(Student {
        id: 1,
        name: String::from("John"),
        email: String::from("john@example.com"),
    });

    repository.save(Student {
        id: 2,
        name: String::from("Mary"),
        email: String::from("mary@example.com"),
    });

    // Getting a student by ID
    if let Some(student) = repository.get_by_id(1) {
        println!("Student found:");
        println!("ID: {}", student.id);
        println!("Name: {}", student.name);
        println!("Email: {}", student.email);
    } else {
        println!("Student not found.");
    }
}
