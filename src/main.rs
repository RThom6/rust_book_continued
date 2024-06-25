use chapter_7_onward::eat_at_restaurant;
use crate::garden::vegetables::Asparagus;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::{self, ErrorKind, Read};
// Nested path
// use std::{cmp::Ordering, io};
// use std::io::{self, Write}; brings std::io and std::io::Write into scope * to bring all public items
pub mod garden; // module garden

pub mod vectors;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    eat_at_restaurant();

    let test = String::from("magic");

    for a in test.chars() {
        println!("{a}");
    }

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 3);
    scores.insert(String::from("Bluer"), 30);

    // Unwrap_or can set this score to 0 if there is no value for it
    // We get an Option<i32> instead of Option<i32> by using .copied()
    let blue_score = scores.get("Blue").copied().unwrap_or(0);

    println!("{blue_score}");

    // Can iterate over a HashMap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let s = String::from("string");
    let mut map = HashMap::new();
    
    // Here the hashmap has taken ownership of s and s is no longer valid
    // Can use &s
    map.insert(s, 4);

    let some_string = "a magic word in a string";
    for x in some_string.split_whitespace() {
        let count = map.entry(x.to_string()).or_insert(0);
        // Dereference mutable reference so we can assign value
        // * causes it to point directly to the referenced data as '.entry()' returns a reference
        *count += 1;
    }

    let greeting_file_result = File::open("test.txt");

    // Opening a file can fail (as it will here)
    // We can match on different error types
    let _file = match greeting_file_result {
        Ok(_file) => _file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };


}

fn _read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    // ? propagates errors
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn _read_username_2() -> Result<String, io::Error> {
    // Opens file, creates string and reads file into it right away
    fs::read_to_string("hello.txt")
}