fn main() {
    let str = r"The main said _Hello World \n \t ' ";
    println!("{}", str);

    let json_str = "{
        \"name\": \"Melvin\"
        \"age\": 22,
        \"sex\": Male
        }";

    let json_str = r#"{
        "name": "Melvin"
        "age": 22,
        "sex": Male
        }"#;

    let str = r###"Hello"## World!"###;
}
