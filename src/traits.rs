pub trait Summary {
    // Interface default behaviour
    fn summarize(&self) -> String {
        String::from("(Read more..)")
    }
}