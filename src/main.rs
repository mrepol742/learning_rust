// fn vowels(words: &str) -> u8 {
//     let vowels_count: usize = words
//         .chars()
//         .into_iter()
//         .filter(|x: &char| (*x == 'a') | (*x == 'e') | (*x == 'i') | (*x == 'o') | (*x == 'u'))
//         .count();

//     vowels_count as u8
// }

// fn main() {
//     let affan = "affan".to_string();
//     println!("{}: {:?}", affan, vowels(&affan));

//      println!("Ferris: {}", vowels("Ferris"));
// }

// fn length_str(x: &str) {
//     println!("length of the string is {} is {}", x, x.len());
// }

// fn main() {
//     let box_str = Box::new("Hello");
//     length_str(&box_str);

//     length_str("Hello Rust");
// }
//

fn square_values(num_vec: &[i32]) {
    for i in num_vec {
        println!("The square is {}", i);
    }
}

fn main() {
    let values_vec = vec![1,2,3,4,5,6];
    let values_array = [1,2,3,4,5,6];

    square_values(&values_vec);
    square_values(&values_array);
}
