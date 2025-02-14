use std::fmt::format;

fn main() {
    println!("Hello, world!");


    // Compund data types - strings
    let some_string = "some string";
    println!("{}", some_string);

    let mut growable_string= String::from("hello my new string!");
    println!("growable string: {}\n", growable_string);
    
    growable_string.push('d');
    println!("{}", growable_string);
    
    growable_string.pop();
    println!("{}", growable_string);


    growable_string.push_str("dupa blada wyglad ma mizerny!");
    println!("{}", growable_string);

    // basic func on strings

    println!("basic functions on strings,
    is_empty(): {},
    length: {},
    Bytes: {},
    Contains 'use' {}",
growable_string.is_empty(),
growable_string.len(),
growable_string.capacity(),
growable_string.contains("use")
);

    // string addition / trim etc
    growable_string.push_str("     ");
    let str_len = growable_string.trim().len();
    println!("{}", str_len);

    let number = 6;
    let num_str: String = number.to_string();
    println!("{}", num_str);
    println!("is the number really a string {}", number.to_string() == "6");


    // converting
    let some_char = 'a';
    let char_str: String = some_char.to_string();
    println!("{}", char_str);

    // new empty strings
    let mut empty_string = String::new();
    empty_string.push_str("dupa");
    println!("{}", empty_string);

    // format macro
    let s1: String = "Bolek".to_string();
    let s2: String = "Lolek".to_string();
    let s3: String = format!("name is {} {}", s1, s2);
    println!("{}", s3);




}
