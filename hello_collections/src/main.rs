use std::collections::HashMap;

fn main() {
    
    // Vector
    let mut v = vec![1,2,3];
    v.push(4);

    let v2 : u32 = v[2];
    println!("{}", v2);

    let second : Option<&u32> = v.get(2);
    match second {
        Some(second) => println!("The third element is {second}"),
        None => println!("There is no second element."),
    }

    for i in &v {
        println!("{i}");
    }

    // String
    let s = String::from("Hello");
    // s.push_str("World");

    let s1 = String::from(", World");
    // println!("{}", s);

    let s2 = s + &s1;
    println!("{}", s2);

    let s3 = String::from("tic");
    let s4 = String::from("tac");
    let s5 = String::from("toe");

    let s = format!("{s3}-{s4}-{s5}");
    println!("{}", s);

    for c in s.chars() {
        println!("{c}");
    }

    for b in s.bytes() {
        println!("{b}");
    }


    // Hash Maps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 25);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
