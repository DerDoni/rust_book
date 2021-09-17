use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut map = HashMap::new();
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);

    let field_name = String::from("Favorite Color");
    let field_value = String::from("blue");

    map.insert(field_name, field_value);
    println!("{}", s2);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    scores.iter().for_each(|(key, value)| {
        println!("{}: {}", key, value);
    });
}
