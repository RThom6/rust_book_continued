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
    fn _x(&self) -> &T {
        &self.x
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