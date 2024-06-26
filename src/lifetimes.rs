use std::fmt::Display;

pub fn example() {
    let string1 = String::from("Lng string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // novel doesn't go out of scope until after ImportantExcerpt does so this is all good
    let novel = String::from("Call me Jack. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    i.announce_and_return_part("e");

    // Static lifetime - Can live for the entire program
    let _s: &'static str = "I have a static lifetime.";

    let string3 = String::from("Lng string is long");
    let string4 = String::from("xyz");

    longest_with_an_announcement(&string3, &string4, "e");
}

// Generic type, trait bound and lifetime in one function
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Very strange stuff
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Need the first 2 lifetime declarations but nothing else as they fall under elision rules
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}