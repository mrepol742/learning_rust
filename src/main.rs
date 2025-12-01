// fn main() {
//     let cancer = true;
//     let smoking = false;

//     // match cancer {
//     //     true => match smoking {
//     //         true => println!("Your cancer is likely due to smoking!"),
//     //         false => println!("Your cancer is not due to smoking and therefore it may have some other reasons.")
//     //     },
//     //     false => match smoking {
//     //         true => println!("Smoking is dangerous and may cause cancer"),
//     //         false => println!("You do not have any disease")
//     //     }
//     // }

//     match (cancer, smoking) {
//         (true, true) => println!("Your cancer is likely due to smoking!"),
//         (true, false) => println!(
//             "Your cancer is not due to smoking and therefore it may have some other reasons."
//         ),
//         (false, true) => println!("Smoking is dangerous and may cause cancer"),
//         (false, false) => println!("You do not have any disease"),
//     }
// }

// fn main() {
//     let responses = vec![Ok(100), Err("Client Error"), Ok(300), Err("Server Error")];
//     let result: Result<Vec<_>, &str> = responses.into_iter().collect();
//     println!("{:?}", result);
// }

// use std::collections::HashMap;
// #[derive(Debug)]

// struct Person {
//     name: String,
//     age: u32,
// }

// fn persons_by_name(persons: Vec<Person>) -> HashMap<String, Person> {
//     persons.into_iter().map(|p| (p.name.clone(), p)).collect()
// }

// fn main() {
//     let person_1 = Person {
//         name: "Melvin".to_string(),
//         age: 22,
//     };

//     let person_2 = Person {
//         name: "Jones".to_string(),
//         age: 15,
//     };

//     let person_3 = Person {
//         name: "Repol".to_string(),
//         age: 17,
//     };

//     let persons = vec![person_1, person_2, person_3];
//     let person_hash = persons_by_name(persons);
//     for (name, details) in &person_hash {
//         println!("Person {:?} has the details of {:?}", name, details);
//     }
// }

fn main() {
    for i in (0..=10).rev() {
        println!("{}", i);
    }
}
