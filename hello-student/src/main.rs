struct Student {
    name: String,
    class: i8,
    rollno: String,
    school: String,
}

fn main() {

    let student = Student {
        name:"Mukesh Dutt".to_string(),
        class: 11,
        rollno: "BB38483".to_string(),
        school: "Xyz".to_string()
    };

    println!("Hello {},", student.name);
    println!("Your class:{}, RollNumber:{} and School: {}", student.class, student.rollno, student.school);
}
