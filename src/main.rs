use learning_rust::Student;

fn main() {
    // let new_student: Student = Student {
    //     id: 1,
    //     name: "Melvin".to_string(),
    //     age: 23
    // };
    //

    let new_student = Student::new("melvin".to_string())
        .unwrap_or_default();
    println!("{:?}", new_student);

    let new_student = Student::default();
    println!("{:?}", new_student);

    let new_student = Student{
        age: 12,
        ..Default::default()
    };
    println!("{:?}", new_student);
}
