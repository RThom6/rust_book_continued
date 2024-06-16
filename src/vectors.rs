pub fn direction_and_magnitude() {
    let _v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3]; // i32 type as it is the default of integers 

    v.push(7); // v must be mutable to push onto it

    let third: &i32 = &v[2];
    println!("Third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Third element is {third}"),
        None => println!("There is no third element."),
    }

    // &v[i] does not work if index i is empty, program crash
    // v.get(i) will return None allowing the program to continue

    let mut v = vec![1, 2, 3, 4, 5];
    let _first = &v[0];
    v.push(6);
    // println!("{first}"); // This won't work, borrowing rules

    // Iterating over a vector
    for i in &v {
        println!("{i}");
    }

    // Can iterate all elements by 12
    for i in &mut v {
        *i += 12; // * is the dereference operator, stops i from taking ownership of the value
    }

    // Vectors can only store one type at a time, but can use enums to change this
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // This requires knowing the exhaustive number of types your program wants a vector to be able to hold
    let row = vec![
        SpreadsheetCell::Text(String::from("Text")),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.6),
    ];

}