fn main() {
    // let s1 = String::from("Hello");
    // let s2 = "world";

    // let s3 = s1 + s2;
    // println!("{}", s3);

    // let s1 = String::from("Hello");
    // let s2 = String::from("world");

    // let s3 = s1 + &s2;
    // println!("{} {}", s3, s2);

    let s1 = String::from("Hello");
    let s2 = "world";
    let s3 = String::from("From Rust");
    let s4 = s1 + &s2 + &s3;
    println!("{} {} {}", s4, s2, s3);
}