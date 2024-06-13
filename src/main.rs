use crate::garden::vegetables::Asparagus;

pub mod garden; // module garden

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
