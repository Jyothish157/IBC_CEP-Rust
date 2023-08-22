struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    
    students.push(Student {
        name: String::from("ABI"),
        email: String::from("abi@gmail.com"),
        phone: String::from("7845612798"),
        id: 1,
    });

    students.push(Student {
        name: String::from("BALA"),
        email: String::from("bala@gmail.com"),
        phone: String::from("9638514796"),
        id: 2,
    });

    students.push(Student {
        name: String::from("RAM"),
        email: String::from("ram@gmail.com"),
        phone: String::from("9342215630"),
        id: 3,
    });

    students.push(Student {
        name: String::from("KRITHI"),
        email: String::from("krithi@example.com"),
        phone: String::from("8514936542"),
        id: 4,
    });

    students.push(Student {
        name: String::from("EMIL"),
        email: String::from("emil@example.com"),
        phone: String::from("9962645912"),
        id: 5,
    });

    
    let index = 3; 
    match students.get(index) {
        Some(student) => {
            println!("Student ID: {}", student.id);
            println!("Name: {}", student.name);
            println!("Email: {}", student.email);
            println!("Phone: {}", student.phone);
        }
        None => {
            println!("Student not found at index {}", index);
        }
    }
}
