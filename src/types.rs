use std::fmt::Display;

// Uses generic type to find the largest of some thing, could be i32 or char etc
pub fn largest<T>(list: &[T]) -> &T {
    let largest = &list[0];

    // Only works for types that implement PartialOrd
    // for item in list {
    //     if item > largest {
    //         largest = item;
    //     }
    // }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // Returns a reference to x
    fn _x(&self) -> &T {
        &self.x
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Can write constraints for only specific types
impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Here x and y can be different types or the same
struct _Point2<T, U> {
    x: T,
    y: U,
}

pub fn not_main() {
    // Type T must match in both x and y 
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.5 };

    println!("{:?}", integer.x);
    println!("{:?}", float.y);
}