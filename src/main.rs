use chapter_7_onward::eat_at_restaurant;
use crate::garden::vegetables::Asparagus;
// Nested path
// use std::{cmp::Ordering, io};
// use std::io::{self, Write}; brings std::io and std::io::Write into scope * to bring all public items
pub mod garden; // module garden

pub mod vectors;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    eat_at_restaurant();
}